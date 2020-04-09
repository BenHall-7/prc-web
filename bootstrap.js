function fetchLabels() {
  return fetch("https://raw.githubusercontent.com/ultimate-research/param-labels/master/ParamLabels.csv")
    .then(res => res.text())
}

import("./pkg").then(module => {
  fetchLabels()
    .then(res => {
      module.load_labels(res)
      module.run_app();
    })
    .catch(_ => {
      module.run_app();
    });
});
