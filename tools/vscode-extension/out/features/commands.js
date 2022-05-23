"use strict";
exports.__esModule = true;
exports.reviewQuestionCallback = exports.solveQuestionCallback = exports.refreshCodeLensCallback = exports.codelensDisposable = void 0;
var vscode_1 = require("vscode");
var codeLensProvider_1 = require("./codeLensProvider");
var child_process_1 = require("child_process");
var codeLensProvider = new codeLensProvider_1.MQSCodeLensProvider();
var mqsExecutable = vscode_1.workspace.getConfiguration('mqs').get("mqsExecutablePath");
// refreshes codelens
var refreshCodeLensCallback = function () {
    if (exports.codelensDisposable)
        exports.codelensDisposable.dispose();
    exports.codelensDisposable = vscode_1.languages.registerCodeLensProvider({ language: "mqs", scheme: "file" }, codeLensProvider);
};
exports.refreshCodeLensCallback = refreshCodeLensCallback;
// solves a question
var solveQuestionCallback = function (uri, name) {
    try {
        (0, child_process_1.spawnSync)(mqsExecutable, [uri.fsPath], { encoding: 'utf-8', shell: true });
    }
    catch (e) {
        console.warn(e);
        vscode_1.window.showWarningMessage("Executing mqs failed. Please check if setting 'mqs.mqsExecutablePath' is valid.");
    }
    (0, exports.refreshCodeLensCallback)();
};
exports.solveQuestionCallback = solveQuestionCallback;
// reviews a question
var reviewQuestionCallback = function (uri, name) {
    try {
        // run mqs review
        var args = [uri.fsPath, '--review', "--at=".concat(name)];
        var r = (0, child_process_1.spawnSync)(mqsExecutable, args, { encoding: 'utf-8', shell: true });
        if (r.error) {
            vscode_1.window.showErrorMessage("Failed to review '?".concat(name, "'. See interpreter output for more information."));
            return;
        }
        // prepare stdout html view
        var text = "<pre>" + r.stdout.trim() + "</pre>";
        // let text = r.stdout.trim()
        // 	.replaceAll('\n', " \\\n")
        // 	.replaceAll("    ", "&emsp;&emsp;")
        // 	.replaceAll(/\`(.*)\`/g, (substr, ...args) => {
        // 		return `<pre>\`${args[0]}\`</pre>`;
        // 	});
        // create webview
        var panel = vscode_1.window.createWebviewPanel("markdown.preview", "Review of `?".concat(name, "`"), { viewColumn: vscode_1.ViewColumn.Beside });
        panel.webview.html = text;
    }
    catch (e) {
        console.warn(e);
        vscode_1.window.showWarningMessage("Executing mqs failed. Please check if setting 'mqs.mqsExecutablePath' is valid.");
    }
    (0, exports.refreshCodeLensCallback)();
};
exports.reviewQuestionCallback = reviewQuestionCallback;
