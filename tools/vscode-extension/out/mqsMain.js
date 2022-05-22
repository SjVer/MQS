"use strict";
exports.__esModule = true;
exports.activate = void 0;
var vscode = require("vscode");
var child_process_1 = require("child_process");
var cmd = require("./features/commands");
function activate(context) {
    if (!vscode.workspace.getConfiguration("mqs").get("enableLanguageFeatures"))
        return;
    // set status bar
    var statusbarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 0);
    var output = "MQS: Version unknown";
    try {
        output = (0, child_process_1.execSync)(vscode.workspace.getConfiguration("mqs").get("mqsExecutablePath") + " --version").toString();
    }
    catch (error) { }
    statusbarItem.text = output.replace("mqs ", "MQS: ");
    statusbarItem.show();
    // set commands for codelens
    context.subscriptions.push(vscode.commands.registerCommand("mqs.solveQuestion", cmd.solveQuestionCallback));
    context.subscriptions.push(vscode.commands.registerCommand("mqs.reviewQuestion", cmd.reviewQuestionCallback));
    // set and subscribe codelens
    cmd.refreshCodeLensCallback();
    context.subscriptions.push(cmd.codelensDisposable);
}
exports.activate = activate;
