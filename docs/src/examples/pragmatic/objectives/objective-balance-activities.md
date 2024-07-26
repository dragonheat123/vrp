# Balance activities with fleet minimization

<details>
    <summary>Problem</summary><p>

```json
{{#include ../../../../../examples/data/pragmatic/objectives/berlin.balance-activities.problem.json}}
```

</p></details>

<details>
    <summary>Solution</summary><p>

```json
{{#include ../../../../../examples/data/pragmatic/objectives/berlin.balance-activities.solution.json}}
```

</p></details>

</br>

<div id="geojson" hidden>
{{#include ../../../../../examples/data/pragmatic/objectives/berlin.balance-activities.solution.geojson}}
</div>

<div id="map"></div>

This objective balances amount of activities and minimizes fleet usage at the same time:

```json
{{#include ../../../../../examples/data/pragmatic/objectives/berlin.balance-activities.problem.json:1004:1025}}
```

Only three vehicles used approximately 16 jobs per vehicle. If you remove `minimize-tours`, results should be similar
to results on previous page.