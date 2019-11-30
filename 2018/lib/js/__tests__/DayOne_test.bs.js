// Generated by BUCKLESCRIPT VERSION 4.0.7, PLEASE EDIT WITH CARE
'use strict';

var Jest = require("@glennsl/bs-jest/lib/js/src/jest.js");
var DayOneData$AdventOfCode2018 = require("../src/day-1-chronal-calibration/DayOneData.bs.js");
var CalibrateOne$AdventOfCode2018 = require("../src/day-1-chronal-calibration/CalibrateOne.bs.js");
var CalibrateTwo$AdventOfCode2018 = require("../src/day-1-chronal-calibration/CalibrateTwo.bs.js");

Jest.test("handles positive increment", (function (param) {
        return Jest.Expect[/* toEqual */12](1, Jest.Expect[/* expect */0](CalibrateOne$AdventOfCode2018.deviceFrequency("+1", undefined, undefined, undefined, /* () */0)[/* result */2]));
      }));

Jest.test("handles multiple positive increments", (function (param) {
        return Jest.Expect[/* toEqual */12](2, Jest.Expect[/* expect */0](CalibrateOne$AdventOfCode2018.deviceFrequency("+1, +1", undefined, undefined, undefined, /* () */0)[/* result */2]));
      }));

Jest.test("handles mixed values", (function (param) {
        return Jest.Expect[/* toEqual */12](0, Jest.Expect[/* expect */0](CalibrateOne$AdventOfCode2018.deviceFrequency("+1, +1, -2", undefined, undefined, undefined, /* () */0)[/* result */2]));
      }));

Jest.test("handles puzzle input #1", (function (param) {
        return Jest.Expect[/* toEqual */12](411, Jest.Expect[/* expect */0](CalibrateOne$AdventOfCode2018.deviceFrequency(DayOneData$AdventOfCode2018.data, undefined, undefined, undefined, /* () */0)[/* result */2]));
      }));

Jest.test("handles small puzzle input #2", (function (param) {
        return Jest.Expect[/* toEqual */12](0, Jest.Expect[/* expect */0](CalibrateTwo$AdventOfCode2018.findDuplicateFrequency("+1, -1", undefined, undefined, undefined, /* () */0)));
      }));

Jest.test("handles medium puzzle input #2", (function (param) {
        return Jest.Expect[/* toEqual */12](10, Jest.Expect[/* expect */0](CalibrateTwo$AdventOfCode2018.findDuplicateFrequency("+3, +3, +4, -2, -4", undefined, undefined, undefined, /* () */0)));
      }));

Jest.test("handles medium puzzle input #2", (function (param) {
        return Jest.Expect[/* toEqual */12](5, Jest.Expect[/* expect */0](CalibrateTwo$AdventOfCode2018.findDuplicateFrequency("-6, +3, +8, +5, -6", undefined, undefined, undefined, /* () */0)));
      }));

Jest.test("handles medium puzzle input #2", (function (param) {
        return Jest.Expect[/* toEqual */12](14, Jest.Expect[/* expect */0](CalibrateTwo$AdventOfCode2018.findDuplicateFrequency("+7, +7, -2, -7, -4", undefined, undefined, undefined, /* () */0)));
      }));

Jest.Skip[/* test */0]("handles full puzzle input #2 - (long running task)", (function (param) {
        return Jest.Expect[/* toEqual */12](56360, Jest.Expect[/* expect */0](CalibrateTwo$AdventOfCode2018.findDuplicateFrequency(DayOneData$AdventOfCode2018.data, undefined, undefined, undefined, /* () */0)));
      }));

/*  Not a pure module */
