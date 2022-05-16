import { CodeLensProvider, TextDocument, CodeLens, Command, Uri, CancellationToken, ProviderResult } from "vscode";

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
		let unnamedQuestionsCount: number = 0;
		let inComment: boolean = false;

		for(let ln = 0; ln < document.lineCount; ln++) {
			const line = document.lineAt(ln);

			if(!inComment && questionRegex.test(line.text)) {
				// figure out name
				const match = /(?!\-\-\s*)\?\s*([a-zA-Z_][a-zA-Z0-9_]*)?/gm.exec(line.text);
				const name = match[1] ? match[1] : (unnamedQuestionsCount++).toString();

				// push solve codelens
				lenses.push({
					isResolved: true,
					range: line.range,
					command: createMQSSolveCommand(document.uri, name)
				});
				
				// push review result codelens
				lenses.push({
					isResolved: false,
					range: line.range,
				});
				
				ln--; // idk why but it'll skip the next line if ln isn't decremented
			}

			if(commentStartRegex.test(line.text)) inComment = true;
			if(inComment && commentEndRegex.test(line.text)) inComment = false;
		}
		
		return lenses;
	}

	resolveCodeLens(lens: CodeLens, token: CancellationToken): ProviderResult<CodeLens> {
		if(lens.isResolved) return null;
	}
}