import { randomInt } from "crypto";
import { CodeLensProvider, TextDocument, CodeLens, Command, Uri, Range, TextEditor, window } from "vscode";
import { MQSResult, getMQSResult, MQSResultValue } from './resultDatabase';

const questionRegex = /(?:^|\s*)\?\s*([a-zA-Z_][a-zA-Z0-9_]*)?/gm; 
const commentStartRegex = /--\*(?!.*\*--)/gm;
const commentEndRegex = /\*--/gm;

function createMQSSolveCommand(uri: Uri, name: string): Command {
	const result: MQSResult = {
		key: { uri: uri, name: name },
		value: { correct: randomInt(2) == 0 }
	};

	return {
		title: "Solve question",
		tooltip: `Solve question "?${name}"`,
		command: 'mqs.setResult',
		arguments: [result]
	};
}

function createMQSReviewCommand(uri: Uri, name: string): Command {
	const result: MQSResultValue = getMQSResult({ uri: uri, name: name });
	return {
		title: `Review result (${result.correct ? "correct" : "incorrect"})`,
		tooltip: `Review the result of question "?${name}"`,
		command: null
	};
}

function createRenameCommand(name: string, range: Range): Command {
	return {
		title: "Rename (question already exists)",
		command: "mqs.renameQuestion",
		arguments: [window.activeTextEditor, name, range],
		tooltip: `Rename the question to "${name}1"`
	};
}

// keep track of the lines with questions and their contents
// so that changes to those can be detected
let questionLines: { uri: Uri, line: number, text: string }[] = []

function getQuestionLine(uri: Uri, line: number) {
	for(var entry of questionLines)
		if(entry.line == line && entry.uri == uri) return entry;
	return null;
}

export class MQSCodeLensProvider implements CodeLensProvider {

	async provideCodeLenses(document: TextDocument): Promise<CodeLens[]> {

		let lenses: CodeLens[] = [];
		let unnamedQuestionsCount: number = 0;
		let questionNames: String[] = [];
		let inComment: boolean = false;

		for(let ln = 0; ln < document.lineCount; ln++) {
			const line = document.lineAt(ln);

			if(!inComment && questionRegex.test(line.text)) {
				// figure out name
				const match = /\?\s*([a-zA-Z_][a-zA-Z0-9_]*)?/gm.exec(line.text);
				const name = match[1] ? match[1] : (unnamedQuestionsCount++).toString();

				// if we already pushed codelenses for this line ignore it
				if(getQuestionLine(document.uri, ln)) continue;

				// push duplicate name codelens?
				if(questionNames.includes(name))
				{
					lenses.push({
						isResolved: true,
						range: line.range,
						command: createRenameCommand(name, line.range)
					});
					continue;
				}
				
				// push solve codelens
				lenses.push({
					isResolved: true,
					range: line.range,
					command: createMQSSolveCommand(document.uri, name)
				});
				
				// push review result codelens
				if(getMQSResult({uri: document.uri, name: name})) lenses.push({
					isResolved: true,
					range: line.range,
					command: createMQSReviewCommand(document.uri, name)
				});
				
				questionNames.push(name);
				questionLines.push({uri: document.uri, line: ln, text: line.text});
				ln--; // idk why but it'll skip the next line if ln isn't decremented
			}

			if(commentStartRegex.test(line.text)) inComment = true;
			if(inComment && commentEndRegex.test(line.text)) inComment = false;
		}
		
		return lenses;
	}
	
	public lineChanged(document: TextDocument, line: number) {
		const qline = getQuestionLine(document.uri, line);
		if(qline && document.lineAt(line).text != qline.text)
			delete qline; 
	}
}