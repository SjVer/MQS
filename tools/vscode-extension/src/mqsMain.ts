import * as vscode from "vscode";
import { execSync } from "child_process";
import * as cmd from './features/commands';
// import { Disposable, languages, Range, TextEditor } from 'vscode';
// import { MQSResult, setMQSResult } from './features/resultDatabase';
// import { MQSCodeLensProvider } from "./features/codeLensProvider";

export function activate(context: vscode.ExtensionContext): any {
	if(!vscode.workspace.getConfiguration("mqs").get<boolean>("enableLanguageFeatures")) return;

	// set status bar
	let statusbarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 0);
	let output: String = "MQS: Version unknown";
	try { output = execSync(vscode.workspace.getConfiguration("mqs").get<string>("mqsExecutablePath") + " --version").toString(); }
	catch (error) {}
	statusbarItem.text = output.replace("mqs ", "MQS: ");
	statusbarItem.show();

	// set commands for codelens
	context.subscriptions.push(vscode.commands.registerCommand("mqs.setResult", cmd.setMQSResultCallback));
	context.subscriptions.push(vscode.commands.registerCommand("mqs.refreshCodeLens", cmd.refreshCodeLensCallback));
	context.subscriptions.push(vscode.commands.registerCommand("mqs.renameQuestion", cmd.renameQuestionCallback));
	context.subscriptions.push(vscode.workspace.onDidChangeTextDocument(cmd.textChangedCallback));

	// set and subscribe codelens
	cmd.refreshCodeLensCallback();
	context.subscriptions.push(cmd.codelensDisposable);
}