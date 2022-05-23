import { workspace, window } from 'vscode';
import { spawnSync } from 'child_process';

const mqsQuickInfoExecutable = workspace.getConfiguration('mqs').get<string>("mqsQuickInfoExecutablePath");

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
			case QuickInfoMode.Json: return r.status === 0 ? JSON.parse(r.stdout) : {}; 
		}

	} catch (e) {
		console.warn(`command: ${command} ${args}`);
		console.warn(e);
		window.showWarningMessage(`Executing mqs-quickinfo failed. Please check if setting 'mqs.mqsQuickInfoExecutablePath' is valid.`)
	}
}