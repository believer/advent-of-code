var bags = {};
var bagsC = {};

input.split("\n").forEach((rule) => {
  let [color, b] = rule.split("bags contain");

  b.split(",")
    .map((l) => l.trim())
    .map((l) => l.replace(/bags?[,.]?/g, "").trim())
    .forEach((l) => {
      let n = l.match(/\d+/);
      let c = l.replace(/\d+/g, "").trim();

      if (!n) {
        bags[color.trim()] = 0;
        return;
      }

      let num = n[0];

      if (!bags[c]) {
        bags[c] = {};
      }

      if (!bagsC[color.trim()]) {
        bagsC[color.trim()] = {};
      }

      bags[c][color.trim()] = parseInt(num, 10);
      bagsC[color.trim()][c] = parseInt(num, 10);
    });
});

var holdsGold = {};

var searcher = (color) => {
  for (c in bags[color]) {
    holdsGold[c] = 0;
    searcher(c);
  }
};

var cost = (color) => {
  var total = 0;

  if (!bagsC[color]) {
    return total;
  }

  for (let [c, ct] of Object.entries(bagsC[color])) {
    total += ct;
    total += ct * cost(c);
  }

  return total;
};

// searcher("shiny gold");
console.log(cost("shiny gold"));

// console.log(Object.keys(holdsGold).length);
