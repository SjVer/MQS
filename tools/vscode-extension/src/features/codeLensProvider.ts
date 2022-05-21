import { CodeLensProvider, TextDocument, CodeLens, Command, Uri } from "vscode";
import { quickInfo, QuickInfoMode } from './commands';

const questionRegex = /(?:^|\s*)\?\s*([a-zA-Z_][a-zA-Z0-9_]*)?/gm; 
const commentStartRegex = /--\*(?!.*\*--)/gm;
const commentEndRegex = /\*--/gm;

function createMQSSolveCommand(uri: Uri, name: string): Command {
	return {
		title: "Solve question",
		tooltip: `Solve question "?${name}"`,
		command: 'mqs.solveQuestion',
		arguments: [uri, name],
	};
}

function createMQSReviewCommand(uri: Uri, name: string, correct: boolean): Command {
	return {
		title: `Review result (${correct ? "correct" : "incorrect"})`,
		tooltip: `Review the result of question "?${name}"`,
		command: null
	};
}

export class MQSCodeLensProvider implements CodeLensProvider {

	async provideCodeLenses(document: TextDocument): Promise<CodeLens[]> {
		let lenses: CodeLens[] = [];

		let questions = await quickInfo(QuickInfoMode.Json, "get-questions", document.uri.fsPath) as object;
		for(var name in questions) {
			const line = document.lineAt(questions[name] - 1);

			// push solve codelens
			lenses.push({
				isResolved: true,
				range: line.range,
				command: createMQSSolveCommand(document.uri, name)
			});

			// push review result codelens
			if(await quickInfo(QuickInfoMode.ExitCode, "can-review-question", document.uri.fsPath, name) === 0) {
				const is_correct = await quickInfo(QuickInfoMode.ExitCode, "question-is-true", document.uri.fsPath, name) === 0;

				lenses.push({
					isResolved: true,
					range: line.range,
					command: createMQSReviewCommand(document.uri, name, is_correct)
				});
			}
		}
		
		return lenses;
	}
}