import { isAbsolute } from 'path';
import * as vscode from 'vscode';
import { ThrottledDelayer } from '../utils/async';
import { callMQSLint, MQSDiagnostics, MQSLintType } from '../utils/lint';

export default class MQSValidationProvider {

	private validationEnabled: boolean;
	private pauseValidation: boolean;

	private documentListener: vscode.Disposable | null = null;
	private diagnosticCollection?: vscode.DiagnosticCollection;
	private delayers?: { [key: string]: ThrottledDelayer<void> };

	constructor() {
		this.validationEnabled = true;
		this.pauseValidation = false;
		this.diagnosticCollection = vscode.languages.createDiagnosticCollection();
		this.loadConfiguration();	
	}

	public activate(subscriptions: vscode.Disposable[]) {
		this.pauseValidation = false;
		this.diagnosticCollection = vscode.languages.createDiagnosticCollection();
		subscriptions.push(this);
		subscriptions.push(vscode.workspace.onDidChangeConfiguration(() => this.loadConfiguration()));

		vscode.workspace.onDidOpenTextDocument(this.triggerValidate, this, subscriptions);
		vscode.workspace.onDidCloseTextDocument((textDocument) => {
			this.diagnosticCollection!.delete(textDocument.uri);
			if (this.delayers) {
				delete this.delayers[textDocument.uri.toString()];
			}
		}, null, subscriptions);
	}

	public dispose(): void {
		if (this.diagnosticCollection) {
			this.diagnosticCollection.clear();
			this.diagnosticCollection.dispose();
		}
		if (this.documentListener) {
			this.documentListener.dispose();
			this.documentListener = null;
		}
	}

	private async loadConfiguration(): Promise<void> {
		this.validationEnabled = true;

		this.delayers = Object.create(null);
		if (this.documentListener) {
			this.documentListener.dispose();
			this.documentListener = null;
		}
		this.diagnosticCollection!.clear();
		if (this.validationEnabled) {
			this.documentListener = vscode.workspace.onDidChangeTextDocument((e) => { this.triggerValidate(e.document); });
			// Configuration has changed. Reevaluate all documents.
			vscode.workspace.textDocuments.forEach(this.triggerValidate, this);
		}
	}

	private async triggerValidate(textDocument: vscode.TextDocument): Promise<void> {
		if (textDocument.languageId !== 'mqs' || this.pauseValidation || !this.validationEnabled) return;
		
		let key = textDocument.uri.toString();
		let delayer = this.delayers![key];
		if (!delayer) {
			delayer = new ThrottledDelayer<void>(250);
			this.delayers![key] = delayer;
		}
		
		delayer.trigger(() => this.doValidate(textDocument));
	}

	private doValidate(textDocument: vscode.TextDocument): Promise<void> {
		return new Promise<void>(resolve => {
			const executable = vscode.workspace.getConfiguration("mqs").get<string>("mqsExecutablePath");
			if (!executable) {
				this.showErrorMessage('Failed to get MQS executable. Use the setting \'mqs.mqsExecutablePath\' to configure the MQS executable.');
				this.pauseValidation = true;
				resolve();
				return;
			}
			if (!isAbsolute(executable)) {
				// executable should either be resolved to an absolute path or undefined.
				// This is just to be sure.
				this.showErrorMessage(`'${executable}' is not an absolute path.`);
				return;
			}

			let diagnostics: { [file: string]: vscode.Diagnostic[] } = {};
			callMQSLint(textDocument, MQSLintType.Diagnostics).then((result: MQSDiagnostics) => {
				// success
				this.diagnosticCollection.clear();
				
				result.diagnostics.forEach(diagnostic => {
					if(!diagnostics[diagnostic.position.file]) diagnostics[diagnostic.position.file] = [];

					const start: vscode.Position = new vscode.Position(diagnostic.position.line, diagnostic.position.column);
					const end: vscode.Position = start.translate(0, diagnostic.position.length);

					let vsdiagnostic: vscode.Diagnostic = new vscode.Diagnostic(new vscode.Range(start, end), diagnostic.message);
					vsdiagnostic.source = 'mqs';
					vsdiagnostic.code = diagnostic.code,
					vsdiagnostic.relatedInformation = [];

					// add related information
					diagnostic.related.forEach(info => {
						const start: vscode.Position = new vscode.Position(info.position.line, info.position.column);
						const end: vscode.Position = start.translate(0, info.position.length);

						vsdiagnostic.relatedInformation.push({
							location: { uri: vscode.Uri.file(info.position.file), range: new vscode.Range(start, end) },
							message: info.message,
						});
					});

					// set severity
					if(diagnostic.type == "error") vsdiagnostic.severity = vscode.DiagnosticSeverity.Error;
					else if(diagnostic.type == "warning") vsdiagnostic.severity = vscode.DiagnosticSeverity.Warning;
					else if(diagnostic.type == "note") vsdiagnostic.severity = vscode.DiagnosticSeverity.Information;
					
					diagnostics[diagnostic.position.file].push(vsdiagnostic);
				});

				for (let file in diagnostics) this.diagnosticCollection.set(vscode.Uri.file(file), diagnostics[file]);
				resolve();

			}).catch((error) => {
				// failure
				console.warn(`mqs diagnostics failure in doValidate: ${error}`);
				this.pauseValidation = true;
				resolve();
			});
		});
	}

	// private async showError(error: any, executable: string): Promise<void> {
	// 	let message: string = error.message ? error.message : `Failed to run MQS using path: ${executable}. Reason is unknown.`;
	// 	if (!message) return;
	//
	// 	return this.showErrorMessage(message);
	// }

	private async showErrorMessage(message: string): Promise<void> {
		const openSettings = 'Open Settings';
		if (await vscode.window.showInformationMessage(message, openSettings) === openSettings) {
			vscode.commands.executeCommand('workbench.action.openSettings', 'mqs.mqsExecutablePath');
		}
	}
}