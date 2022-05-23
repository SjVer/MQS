"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (_) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
exports.__esModule = true;
var path_1 = require("path");
var vscode = require("vscode");
var async_1 = require("../utils/async");
var lint_1 = require("../utils/lint");
var MQSValidationProvider = /** @class */ (function () {
    function MQSValidationProvider() {
        this.documentListener = null;
        this.validationEnabled = true;
        this.pauseValidation = false;
        this.diagnosticCollection = vscode.languages.createDiagnosticCollection();
        this.loadConfiguration();
    }
    MQSValidationProvider.prototype.activate = function (subscriptions) {
        var _this = this;
        this.pauseValidation = false;
        this.diagnosticCollection = vscode.languages.createDiagnosticCollection();
        subscriptions.push(this);
        subscriptions.push(vscode.workspace.onDidChangeConfiguration(function () { return _this.loadConfiguration(); }));
        vscode.workspace.onDidOpenTextDocument(this.triggerValidate, this, subscriptions);
        vscode.workspace.onDidCloseTextDocument(function (textDocument) {
            _this.diagnosticCollection["delete"](textDocument.uri);
            if (_this.delayers) {
                delete _this.delayers[textDocument.uri.toString()];
            }
        }, null, subscriptions);
    };
    MQSValidationProvider.prototype.dispose = function () {
        if (this.diagnosticCollection) {
            this.diagnosticCollection.clear();
            this.diagnosticCollection.dispose();
        }
        if (this.documentListener) {
            this.documentListener.dispose();
            this.documentListener = null;
        }
    };
    MQSValidationProvider.prototype.loadConfiguration = function () {
        return __awaiter(this, void 0, void 0, function () {
            var _this = this;
            return __generator(this, function (_a) {
                this.validationEnabled = true;
                this.delayers = Object.create(null);
                if (this.documentListener) {
                    this.documentListener.dispose();
                    this.documentListener = null;
                }
                this.diagnosticCollection.clear();
                if (this.validationEnabled) {
                    this.documentListener = vscode.workspace.onDidChangeTextDocument(function (e) { _this.triggerValidate(e.document); });
                    // Configuration has changed. Reevaluate all documents.
                    vscode.workspace.textDocuments.forEach(this.triggerValidate, this);
                }
                return [2 /*return*/];
            });
        });
    };
    MQSValidationProvider.prototype.triggerValidate = function (textDocument) {
        return __awaiter(this, void 0, void 0, function () {
            var key, delayer;
            var _this = this;
            return __generator(this, function (_a) {
                if (textDocument.languageId !== 'mqs' || this.pauseValidation || !this.validationEnabled)
                    return [2 /*return*/];
                key = textDocument.uri.toString();
                delayer = this.delayers[key];
                if (!delayer) {
                    delayer = new async_1.ThrottledDelayer(250);
                    this.delayers[key] = delayer;
                }
                delayer.trigger(function () { return _this.doValidate(textDocument); });
                return [2 /*return*/];
            });
        });
    };
    MQSValidationProvider.prototype.doValidate = function (textDocument) {
        var _this = this;
        return new Promise(function (resolve) {
            var executable = vscode.workspace.getConfiguration("mqs").get("mqsExecutablePath");
            if (!executable) {
                _this.showErrorMessage('Failed to get MQS executable. Use the setting \'mqs.mqsExecutablePath\' to configure the MQS executable.');
                _this.pauseValidation = true;
                resolve();
                return;
            }
            if (!(0, path_1.isAbsolute)(executable)) {
                // executable should either be resolved to an absolute path or undefined.
                // This is just to be sure.
                _this.showErrorMessage("'".concat(executable, "' is not an absolute path."));
                return;
            }
            var diagnostics = {};
            (0, lint_1.callMQSLint)(textDocument, lint_1.MQSLintType.Diagnostics).then(function (result) {
                // success
                _this.diagnosticCollection.clear();
                result.diagnostics.forEach(function (diagnostic) {
                    if (!diagnostics[diagnostic.position.file])
                        diagnostics[diagnostic.position.file] = [];
                    var start = new vscode.Position(diagnostic.position.line, diagnostic.position.column);
                    var end = start.translate(0, diagnostic.position.length);
                    var vsdiagnostic = new vscode.Diagnostic(new vscode.Range(start, end), diagnostic.message);
                    vsdiagnostic.source = 'mqs';
                    vsdiagnostic.code = diagnostic.code,
                        vsdiagnostic.relatedInformation = [];
                    // add related information
                    diagnostic.related.forEach(function (info) {
                        var start = new vscode.Position(info.position.line, info.position.column);
                        var end = start.translate(0, info.position.length);
                        vsdiagnostic.relatedInformation.push({
                            location: { uri: vscode.Uri.file(info.position.file), range: new vscode.Range(start, end) },
                            message: info.message
                        });
                    });
                    // set severity
                    if (diagnostic.type == "error")
                        vsdiagnostic.severity = vscode.DiagnosticSeverity.Error;
                    else if (diagnostic.type == "warning")
                        vsdiagnostic.severity = vscode.DiagnosticSeverity.Warning;
                    else if (diagnostic.type == "note")
                        vsdiagnostic.severity = vscode.DiagnosticSeverity.Information;
                    diagnostics[diagnostic.position.file].push(vsdiagnostic);
                });
                for (var file in diagnostics)
                    _this.diagnosticCollection.set(vscode.Uri.file(file), diagnostics[file]);
                resolve();
            })["catch"](function (error) {
                // failure
                console.warn("mqs diagnostics failure in doValidate: ".concat(error));
                _this.pauseValidation = true;
                resolve();
            });
        });
    };
    // private async showError(error: any, executable: string): Promise<void> {
    // 	let message: string = error.message ? error.message : `Failed to run MQS using path: ${executable}. Reason is unknown.`;
    // 	if (!message) return;
    //
    // 	return this.showErrorMessage(message);
    // }
    MQSValidationProvider.prototype.showErrorMessage = function (message) {
        return __awaiter(this, void 0, void 0, function () {
            var openSettings;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        openSettings = 'Open Settings';
                        return [4 /*yield*/, vscode.window.showInformationMessage(message, openSettings)];
                    case 1:
                        if ((_a.sent()) === openSettings) {
                            vscode.commands.executeCommand('workbench.action.openSettings', 'mqs.mqsExecutablePath');
                        }
                        return [2 /*return*/];
                }
            });
        });
    };
    return MQSValidationProvider;
}());
exports["default"] = MQSValidationProvider;
