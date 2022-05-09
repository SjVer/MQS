import { Uri } from "vscode";

export interface MQSResultValue { correct: boolean };
export interface MQSResultKey { uri: Uri, name: string }
export interface MQSResult { key: MQSResultKey, value: MQSResultValue }

let MQSResultDatabase: MQSResult[] = []

export function setMQSResult(result: MQSResult): void {
	for(var pair of MQSResultDatabase) {
		if(pair.key.name != result.key.name
		|| pair.key.uri != result.key.uri) continue;
		
		pair.value = result.value;
		return;
	}
	MQSResultDatabase.push(result);
}

export function getMQSResult(key: MQSResultKey): MQSResultValue|null {
	let result: MQSResultValue|null = null;

	for(var pair of MQSResultDatabase) {
		if(pair.key.name != key.name
		|| pair.key.uri != key.uri) continue;

		result = pair.value;
		break;
	}

	return result;
}