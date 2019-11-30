// Generated by BUCKLESCRIPT VERSION 4.0.7, PLEASE EDIT WITH CARE
'use strict';

var Belt_Array = require("bs-platform/lib/js/belt_Array.js");
var Caml_array = require("bs-platform/lib/js/caml_array.js");
var Belt_Option = require("bs-platform/lib/js/belt_Option.js");
var Utils$AdventOfCode2018 = require("../utils/Utils.bs.js");
var Levenshtein$AdventOfCode2018 = require("../utils/Levenshtein.bs.js");

var defaultValue = /* tuple */[
  "",
  "",
  0
];

function findSimilarity(input, i, row) {
  var match = Belt_Array.get(input, i + 1 | 0);
  if (match !== undefined) {
    var next = match;
    return /* tuple */[
            row,
            next,
            Levenshtein$AdventOfCode2018.similarity(row, next)
          ];
  } else {
    return defaultValue;
  }
}

function findBoxes(input) {
  var match = Belt_Option.getWithDefault(Belt_Array.get(Utils$AdventOfCode2018.sortInPlaceWith(Belt_Array.mapWithIndex(input.sort(), (function (param, param$1) {
                      return findSimilarity(input, param, param$1);
                    })), Utils$AdventOfCode2018.sortFloats), 0), defaultValue);
  var allSecond = match[1].split("");
  return Utils$AdventOfCode2018.join(Belt_Array.keepWithIndex(match[0].split(""), (function (firstChar, i) {
                    return Caml_array.caml_array_get(allSecond, i) === firstChar;
                  })), "");
}

exports.defaultValue = defaultValue;
exports.findSimilarity = findSimilarity;
exports.findBoxes = findBoxes;
/* No side effect */
