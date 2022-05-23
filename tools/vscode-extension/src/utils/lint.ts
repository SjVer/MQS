import { TextDocument, Position, workspace, MarkdownString, Uri } from 'vscode';
import { BackwardIterator } from './backwardIterator';
import { spawnSync } from 'child_process';
import { copyFileSync, rmSync, writeFileSync } from 'fs';
import { basename, join } from 'path';
import { tmpdir } from 'os';
import { waitUntil } from './async';

export enum MQSLintType {
	Diagnostics = 'diag',
	Declaration = 'decl',
	Symbols = 'symb',
}

export interface MQSPosition { file: string, line: number, column: number, length: number };
export interface MQSDeclaration { position: MQSPosition };
export interface MQSDiagnostics { diagnostics: { position: MQSPosition, message: string, code?: number, type: string, related: { position: MQSPosition, message: string }[] }[] };
export interface MQSSymbols { symbols: { identifier: string, return_type: string, parameters: string[], variadic: boolean }[] };

let do_log: boolean = workspace.getConfiguration('mqs').get<boolean>('logDebugInfo');
function log(message?: any, ...optionalParams: any[]): void {
	if (do_log) console.log(message, ...optionalParams);
}

let blocked = false;

function parseLocation(loc: string, len?: number): MQSPosition {
	let parts = loc.split(':', 3);
	return {
		file: parts[0],
		line: Number.parseInt(parts[1]) - 1 ?? 0,
		column: Number.parseInt(parts[2]) - 1 ?? 0,
		length: len ?? 0,
	}
}

export async function callMQSLint(document: TextDocument, type: MQSLintType, position?: Position): Promise<any> {
	if(blocked) waitUntil(() => !blocked, 3000);
	blocked = true;

	log(`\n\n\n====== LINT: ${type.toUpperCase()} ======`);

	// copy file so that unsaved changes are included
	let tmpfile = join(tmpdir(), basename(document.fileName) + '.mqslint_tmp');
	copyFileSync(document.fileName, tmpfile);
	writeFileSync(tmpfile, document.getText());
	
	const mqsExec = workspace.getConfiguration('mqs').get<string>('mqsExecutablePath');
	// const workspacefolder = workspace.getWorkspaceFolder(document.uri).uri.fsPath;

	// let pos = position ? `${position.line + 1}:${position.character}` : "0:0";
	let args = [tmpfile, `--lint="${type}"`];

	// workspace.getConfiguration('mqs').get<string[]>('includeSearchPaths').forEach(path =>
	// 	cmd += ` --include=${path}`.replace('${workspaceFolder}', workspacefolder));

	log(`lint cmd: ${mqsExec} ${args.join(' ')}`);
	
	let output: string;
	try { output = spawnSync(mqsExec, args, { encoding: 'utf-8', shell: true }).stdout; }
	catch (error) {
		console.warn(`Failed to execute MQS binary: ${error}`);
		blocked = false;
		return Promise.reject();
	}
	if (tmpfile !== document.fileName) rmSync(tmpfile);
	
	blocked = false;
	
	// remove ansii escape codes
	output = output.replaceAll(RegExp(String.fromCharCode(0x1b) + "\\[([0-9]+;)?[0-9]+m", "g"), '');
	if(tmpfile !== document.fileName) output = output.replaceAll(tmpfile, document.fileName);
	log("lint output: " + output);
	
	let data: any;
	try { data = JSON.parse(output); }
	catch (error) {
		console.warn(`Failed to parse data returned by MQS binary:\n"${error}"\nData: "${output}"`);
		return Promise.reject();
	}

	// change up some data if needed

	if(data['file'] === tmpfile) data['file'] == document.fileName;

	try { switch (type)
	{
		case MQSLintType.Declaration: {
			if (data['invalid']) return undefined;

			let result: MQSDeclaration = {
				position: parseLocation(data['location'], data['length'])
			};

			log(result as any);
			return Promise.resolve(result);
		}
		case MQSLintType.Diagnostics: {
			let result: MQSDiagnostics = { diagnostics: [] };
			data.forEach((error: any) => {
				if(error['invalid']) return;

				// gather related information
				let related: { position: MQSPosition, message: string }[] = [];
				
				error['related'].forEach((info) => {
					related.push({
						position: parseLocation(info['location'], info['length']),
						message: info['message'],
					});
				});

				// create error itself
				result.diagnostics.push({
					position: parseLocation(error['location'], error['length']),
					message: error['message'],
					code: error['code'],
					type: error['severity'],
					related: related,
				});
			});
			log(result as any);
			return Promise.resolve(result);
		}
	} } catch (e) { log(e); }
}

export interface SymbolDoc { main: string, params: string[], ret?: string };

export async function getDocumentation(document: TextDocument, position: Position): Promise<SymbolDoc> {
	// get defenition location of function
	const declaration: MQSDeclaration = await callMQSLint(document, MQSLintType.Declaration, position);
	if (!declaration) return Promise.reject("Symbol declaration not found.");

	let delcdoc: TextDocument = await workspace.openTextDocument(Uri.file(declaration.position.file));
	if (!delcdoc) return Promise.reject("Could not open file " + declaration.position.file);

	let it = new BackwardIterator(delcdoc, declaration.position.column, declaration.position.line);

	// get newline before declaration
	while (it.hasNext()) { if (it.next() == "\n") break; };

	// get full documentation
	let doc: string = "";
	while (it.hasNext()) {
		const c = it.next();
		if (c == "\n" && !doc.startsWith("--?")) {
			// end of documentation, remove last (non-doc) line
			doc = doc.slice(doc.indexOf("--?"));
			
			// test if there's actually a documentation
			if(!doc.startsWith('--?')) doc = "";
			
			break;
		}
		doc = c + doc;
	}
	// replace comment tokens with just newlines
	doc = doc.replace(/\n?\-\-\?\s*/g, "\n") + "\n";
	while (doc.startsWith("\n")) doc = doc.slice(1);


	// get parameters
	let params: string[] = [];
	const paramRegex = /\n\s*\@param\s+([0-9]+)\s+(.*)\n/;
	while (paramRegex.test(doc)) {
		const match = doc.match(paramRegex);
		doc = doc.replace(match[0], "\n");
		params.push(match[2]);
	}
	
	// get return type
	let ret: string = undefined;
	const retRegex = /\n\s*\@return\s+(.*)\n/;
	if (retRegex.test(doc)) {
		const match = doc.match(retRegex);
		doc = doc.replace(match[0], "\n");
		// doc += `\n*@return* - ${match[1]}`;
		ret = match[1];
	}

	return Promise.resolve({ main: doc, params: params, ret: ret });
}

export async function getDocAsMarkdown(document: TextDocument, position: Position): Promise<MarkdownString> {
	const doc: SymbolDoc = await getDocumentation(document, position);

	let text = doc.main
	for (let i = 0; i < doc.params.length; i++)
		text += `\n*@param* \`${i}\` - ${doc.params[i]}`;
	if (doc.ret)
		text += `\n*@return* - ${doc.ret}`;
	
	return new MarkdownString(text.trim().replaceAll("\n", " \\\n"));
}