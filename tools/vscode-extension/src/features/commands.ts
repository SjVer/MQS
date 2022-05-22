import { spawnSync } from 'child_process';
import { Disposable, languages, Uri, ViewColumn, window, workspace, MarkdownString } from 'vscode';
import { MQSCodeLensProvider } from "./codeLensProvider";
import { textToMarkedString } from "./utils/markedTextUtil";
import * as MarkDownIt from "markdown-it";

export let codelensDisposable: Disposable;
const codeLensProvider: MQSCodeLensProvider = new MQSCodeLensProvider();

const mqsQuickInfoExecutable = workspace.getConfiguration('mqs').get<string>("mqsQuickInfoExecutablePath");
const mqsExecutable = workspace.getConfiguration('mqs').get<string>("mqsExecutablePath");

const md = new MarkDownIt({ breaks: true, langPrefix: "mqs-highlighted-", highlight: mdHighlighter });
function mdHighlighter(str: string, lang: string, attrs: string): string {
	// str is contents, lang is language
	if(lang != "mqs") return '';
	let new_str = new MarkdownString().appendCodeblock(str, "mqs");
	return `\`${new_str.value}\``;
}

export enum QuickInfoMode {
	ExitCode,
	Json,
}

export async function quickInfo(mode: QuickInfoMode, command: string, ...args: any[]): Promise<number | object> {
	try {
		let cli_args = [command].concat(args);
		let r = spawnSync(mqsQuickInfoExecutable, cli_args, { encoding: 'utf-8', shell: true });

		switch (mode) {
			case QuickInfoMode.ExitCode: return r.status ?? 0;
			case QuickInfoMode.Json: return JSON.parse(r.stdout) ?? {}; 
		}

	} catch (e) {
		console.warn(e);
		window.showWarningMessage(`Executing mqs-quickinfo failed. Please check if setting 'mqs.mqsQuickInfoExecutablePath' is valid.`)
	}
}

// refreshes codelens
export const refreshCodeLensCallback = () => {
	if(codelensDisposable) codelensDisposable.dispose();

	codelensDisposable  = languages.registerCodeLensProvider(
		{language: "mqs", scheme: "file"}, codeLensProvider);
};

// solves a question
export const solveQuestionCallback = (uri: Uri, name: string) => {
	try {
		spawnSync(mqsExecutable, [uri.fsPath], { encoding: 'utf-8', shell: true });
	} catch (e) {
		console.warn(e);
		window.showWarningMessage("Executing mqs failed. Please check if setting 'mqs.mqsExecutablePath' is valid.");
	}

	refreshCodeLensCallback();
};

// reviews a question
export const reviewQuestionCallback = (uri: Uri, name: string) => {
	try {
		// run mqs review
		let args = [uri.fsPath, '--review', `--at=${name}`];
		let r = spawnSync(mqsExecutable, args, { encoding: 'utf-8', shell: true });
		if(r.error) {
			window.showErrorMessage(`Failed to review '?${name}'. See interpreter output for more information.`);
			return;
		}

		
		// prepare stdout for markdown-to-html translation
		let text = r.stdout.trim().replaceAll("    ", "&emsp;&emsp;").replaceAll(/\`(.*)\`/g, (substr, ...args) => {
			return `\n\`\`\`mqs\n${args[0]}\`\`\`\n`;
		});
		
		// create webview
		const panel = window.createWebviewPanel("markdown.preview", `Review of \`?${name}\``, { viewColumn: ViewColumn.Beside });
		panel.webview.html = md.render(text, process.env);

	} catch (e) {
		console.warn(e);
		window.showWarningMessage("Executing mqs failed. Please check if setting 'mqs.mqsExecutablePath' is valid.");
	}

	refreshCodeLensCallback();
};