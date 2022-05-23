import { Disposable, languages, Uri, ViewColumn, window, workspace, MarkdownString } from 'vscode';
import { MQSCodeLensProvider } from "./codeLensProvider";
import { spawnSync } from 'child_process';

export let codelensDisposable: Disposable;
const codeLensProvider: MQSCodeLensProvider = new MQSCodeLensProvider();

const mqsExecutable = workspace.getConfiguration('mqs').get<string>("mqsExecutablePath");

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

		
		// prepare stdout html view
		let text = "<pre>" + r.stdout.trim() + "</pre>";
		// let text = r.stdout.trim()
		// 	.replaceAll('\n', " \\\n")
		// 	.replaceAll("    ", "&emsp;&emsp;")
		// 	.replaceAll(/\`(.*)\`/g, (substr, ...args) => {
		// 		return `<pre>\`${args[0]}\`</pre>`;
		// 	});
		
		// create webview
		const panel = window.createWebviewPanel("markdown.preview", `Review of \`?${name}\``, { viewColumn: ViewColumn.Beside });
		panel.webview.html = text;

	} catch (e) {
		console.warn(e);
		window.showWarningMessage("Executing mqs failed. Please check if setting 'mqs.mqsExecutablePath' is valid.");
	}

	refreshCodeLensCallback();
};
