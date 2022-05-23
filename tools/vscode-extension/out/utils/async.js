"use strict";
var __extends = (this && this.__extends) || (function () {
    var extendStatics = function (d, b) {
        extendStatics = Object.setPrototypeOf ||
            ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
            function (d, b) { for (var p in b) if (Object.prototype.hasOwnProperty.call(b, p)) d[p] = b[p]; };
        return extendStatics(d, b);
    };
    return function (d, b) {
        if (typeof b !== "function" && b !== null)
            throw new TypeError("Class extends value " + String(b) + " is not a constructor or null");
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
})();
exports.__esModule = true;
exports.waitUntil = exports.ThrottledDelayer = exports.Delayer = exports.Throttler = void 0;
/**
 * A helper to prevent accumulation of sequential async tasks.
 *
 * Imagine a mail man with the sole task of delivering letters. As soon as
 * a letter submitted for delivery, he drives to the destination, delivers it
 * and returns to his base. Imagine that during the trip, N more letters were submitted.
 * When the mail man returns, he picks those N letters and delivers them all in a
 * single trip. Even though N+1 submissions occurred, only 2 deliveries were made.
 *
 * The throttler implements this via the queue() method, by providing it a task
 * factory. Following the example:
 *
 * 		var throttler = new Throttler();
 * 		var letters = [];
 *
 * 		function letterReceived(l) {
 * 			letters.push(l);
 * 			throttler.queue(() => { return makeTheTrip(); });
 * 		}
 */
var Throttler = /** @class */ (function () {
    function Throttler() {
        this.activePromise = null;
        this.queuedPromise = null;
        this.queuedPromiseFactory = null;
    }
    Throttler.prototype.queue = function (promiseFactory) {
        var _this = this;
        if (this.activePromise) {
            this.queuedPromiseFactory = promiseFactory;
            if (!this.queuedPromise) {
                var onComplete_1 = function () {
                    _this.queuedPromise = null;
                    var result = _this.queue(_this.queuedPromiseFactory);
                    _this.queuedPromiseFactory = null;
                    return result;
                };
                this.queuedPromise = new Promise(function (resolve) {
                    _this.activePromise.then(onComplete_1, onComplete_1).then(resolve);
                });
            }
            return new Promise(function (resolve, reject) {
                _this.queuedPromise.then(resolve, reject);
            });
        }
        this.activePromise = promiseFactory();
        return new Promise(function (resolve, reject) {
            _this.activePromise.then(function (result) {
                _this.activePromise = null;
                resolve(result);
            }, function (err) {
                _this.activePromise = null;
                reject(err);
            });
        });
    };
    return Throttler;
}());
exports.Throttler = Throttler;
/**
 * A helper to delay execution of a task that is being requested often.
 *
 * Following the throttler, now imagine the mail man wants to optimize the number of
 * trips proactively. The trip itself can be long, so the he decides not to make the trip
 * as soon as a letter is submitted. Instead he waits a while, in case more
 * letters are submitted. After said waiting period, if no letters were submitted, he
 * decides to make the trip. Imagine that N more letters were submitted after the first
 * one, all within a short period of time between each other. Even though N+1
 * submissions occurred, only 1 delivery was made.
 *
 * The delayer offers this behavior via the trigger() method, into which both the task
 * to be executed and the waiting period (delay) must be passed in as arguments. Following
 * the example:
 *
 * 		var delayer = new Delayer(WAITING_PERIOD);
 * 		var letters = [];
 *
 * 		function letterReceived(l) {
 * 			letters.push(l);
 * 			delayer.trigger(() => { return makeTheTrip(); });
 * 		}
 */
var Delayer = /** @class */ (function () {
    function Delayer(defaultDelay) {
        this.defaultDelay = defaultDelay;
        this.timeout = null;
        this.completionPromise = null;
        this.onResolve = null;
        this.task = null;
    }
    Delayer.prototype.trigger = function (task, delay) {
        var _this = this;
        if (delay === void 0) { delay = this.defaultDelay; }
        this.task = task;
        this.cancelTimeout();
        if (!this.completionPromise) {
            this.completionPromise = new Promise(function (resolve) {
                _this.onResolve = resolve;
            }).then(function () {
                _this.completionPromise = null;
                _this.onResolve = null;
                var result = _this.task();
                _this.task = null;
                return result;
            });
        }
        this.timeout = setTimeout(function () {
            _this.timeout = null;
            _this.onResolve(undefined);
        }, delay);
        return this.completionPromise;
    };
    Delayer.prototype.isTriggered = function () {
        return this.timeout !== null;
    };
    Delayer.prototype.cancel = function () {
        this.cancelTimeout();
        if (this.completionPromise) {
            this.completionPromise = null;
        }
    };
    Delayer.prototype.cancelTimeout = function () {
        if (this.timeout !== null) {
            clearTimeout(this.timeout);
            this.timeout = null;
        }
    };
    return Delayer;
}());
exports.Delayer = Delayer;
/**
 * A helper to delay execution of a task that is being requested often, while
 * preventing accumulation of consecutive executions, while the task runs.
 *
 * Simply combine the two mail man strategies from the Throttler and Delayer
 * helpers, for an analogy.
 */
var ThrottledDelayer = /** @class */ (function (_super) {
    __extends(ThrottledDelayer, _super);
    function ThrottledDelayer(defaultDelay) {
        var _this = _super.call(this, defaultDelay) || this;
        _this.throttler = new Throttler();
        return _this;
    }
    ThrottledDelayer.prototype.trigger = function (promiseFactory, delay) {
        var _this = this;
        return _super.prototype.trigger.call(this, function () { return _this.throttler.queue(promiseFactory); }, delay);
    };
    return ThrottledDelayer;
}(Delayer));
exports.ThrottledDelayer = ThrottledDelayer;
// ===================================================
// refer to https://xmanyou.com/javascript-wait-until-condition-meet-or-timeout/
function waitUntil(condition, timeout, interval) {
    if (timeout === void 0) { timeout = 0; }
    if (interval === void 0) { interval = 50; }
    var waitHandler;
    var timeoutHandler;
    return new Promise(function (resolve, reject) {
        var waitFn = function () {
            if (condition()) {
                if (timeoutHandler) {
                    clearTimeout(timeoutHandler);
                }
                resolve();
            }
            else {
                waitHandler = setTimeout(waitFn, interval);
            }
        };
        // 
        waitHandler = setTimeout(waitFn, interval);
        // timeout, if timeout <=0, never timeout
        if (timeout > 0) {
            timeoutHandler = setTimeout(function () {
                if (waitHandler) {
                    clearTimeout(waitHandler);
                }
                reject({
                    code: "TIMEOUT",
                    message: "timeout"
                });
            }, timeout);
        }
    });
}
exports.waitUntil = waitUntil;
