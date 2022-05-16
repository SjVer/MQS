import { Disposable, languages, Uri, window } from 'vscode';
import { MQSCodeLensProvider } from "./codeLensProvider";

export let codelensDisposable: Disposable;
const codeLensProvider: MQSCodeLensProvider = new MQSCodeLensProvider();

// refreshes codelens
export const refreshCodeLensCallback = () => {
	if(codelensDisposable) codelensDisposable.dispose();

	codelensDisposable  = languages.registerCodeLensProvider(
		{language: "mqs", scheme: "file"}, codeLensProvider);
};

// solves a question
export const solveQuestionCallback = (uri: Uri, name: string) => {
	window.showInformationMessage(`solved ${name} (${uri})`);
	refreshCodeLensCallback();
};