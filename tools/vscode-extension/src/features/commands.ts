import { Disposable, languages, TextDocumentChangeEvent, Range, TextEditor, TextDocumentChangeReason } from 'vscode';
import { MQSResult, setMQSResult } from './resultDatabase';
import { MQSCodeLensProvider } from "./codeLensProvider";

export let codelensDisposable: Disposable;
const codeLensProvider: MQSCodeLensProvider = new MQSCodeLensProvider();

// refreshes codelens
export const refreshCodeLensCallback = () => {
	if(codelensDisposable) codelensDisposable.dispose();

	codelensDisposable  = languages.registerCodeLensProvider(
		{language: "mqs", scheme: "file"}, codeLensProvider);
};

// sets a result
export const setMQSResultCallback = (pair: MQSResult) => {
	setMQSResult(pair); refreshCodeLensCallback();
};

// renames a question
export const renameQuestionCallback = (editor: TextEditor, name: string, range: Range) => {
	const re = /\s*\?\s*[a-zA-Z_][a-zA-Z_0-9]*/gm;
	const offset = editor.document.getText(range).match(re)[0].length;

	// just append '1' to the name
	editor.edit(builder => {
		builder.insert(range.start.translate(0, offset), '1');
	});
};

export const textChangedCallback = (e: TextDocumentChangeEvent) => {
	for(var change of e.contentChanges)
		for(let line = change.range.start.line; line < change.range.start.line; line++)
			codeLensProvider.lineChanged(e.document, line);
}