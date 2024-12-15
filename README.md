# Minimum spanning tree of US regions based on Avocado price correlation

## Overview

For this project,
we use [this](https://www.kaggle.com/datasets/neuromusic/avocado-prices) Avocado price dataset from Kaggle.
Our goal is to answer whether correlated Avocado price movement relate to geographical closeness for US regions.
Our overall approach is then to (1) calculate the Avocado price time series for each US region,
(2) compute the correlation matrix between each pair of regions,
(3) construct a complete undirected weighted graph based on some functions of the correlation,
and (4) compute the minimum spanning tree of the graph and interpret the result.

## Output

### Minimum spanning tree (`output/mst.txt`)

Consider the following final minimum spanning tree.
The first line describes the number of nodes and the number of edges in the tree.
The following lines each describe an edge and the corresponding Avocado price movement correlation between the two regions.
It is apparent that indeed closeness in Avocado price movement point to geographical proximity.

```
45 44
Tampa Orlando 0.987919846884412
Tampa Jacksonville 0.9878206988173541
MiamiFtLauderdale Orlando 0.987751252215062
RaleighGreensboro Charlotte 0.9145680908769609
LosAngeles SanDiego 0.8774425338082265
SouthCarolina Jacksonville 0.8701782421989324
Philadelphia NewYork 0.8402248668068454
Spokane Seattle 0.8182128204220799
SanFrancisco Sacramento 0.8143980326017524
Seattle Portland 0.8127710673921203
Orlando Atlanta 0.7958176046614772
Roanoke RichmondNorfolk 0.7883353706250099
Syracuse Albany 0.7825425575733118
HartfordSpringfield NewYork 0.7626776090900282
SouthCarolina Charlotte 0.7624606902918488
RichmondNorfolk RaleighGreensboro 0.7556455645634852
GrandRapids Detroit 0.7360391474825785
Seattle Boise 0.732317954854521
Columbus Detroit 0.7209218604426058
SouthCarolina NewOrleansMobile 0.7192732050771613
BaltimoreWashington HartfordSpringfield 0.7116608928833514
WestTexNewMexico LasVegas 0.699372396992826
DallasFtWorth Houston 0.6981959268246631
HartfordSpringfield HarrisburgScranton 0.6897203193053816
Boston HartfordSpringfield 0.6566725296876526
Orlando Nashville 0.6367051513935562
Chicago Charlotte 0.5854951643204414
Columbus Roanoke 0.5595307247926484
Louisville Nashville 0.5558074963468045
Indianapolis CincinnatiDayton 0.5470984043407101
LosAngeles RichmondNorfolk 0.5456879383724098
Columbus CincinnatiDayton 0.5379495962003548
Boston NorthernNewEngland 0.5309269323821468
LasVegas Houston 0.5217807556800609
Sacramento Charlotte 0.5185599536499246
BaltimoreWashington Charlotte 0.5165233574399921
Louisville Denver 0.5088200163449876
Syracuse BuffaloRochester 0.5004924186941294
Albany NorthernNewEngland 0.4887220981242512
Seattle RichmondNorfolk 0.4668339482285696
LasVegas RichmondNorfolk 0.44114900188260264
Pittsburgh Charlotte 0.40708003997304015
StLouis Charlotte 0.3785121401432051
Chicago PhoenixTucson 0.28518601917841285
```

### Complete output (`output/output.log`)

The following is the output log (without the final MST).
As outlined in the log,
we first get the relevant columns from the dataset.
We then convert the long form data to wide form.
Finally,
we calculate the correlation matrix for all pairs of regions.
All dataframe operations are done using `polars`.

We then use `petgraph` and construct the complete graph based on the correlation between pairs.
Since we want highly correlated nodes to be closer (have less weight on the edge) for the MST,
we set the weight of an edge to be `1 / corr^2`.
We then find the MST of the induced complete graph,
at which point we export the MST into the edge list format.

```
    Finished `release` profile [optimized] target(s) in 0.19s
     Running `target/release/avocado`
[2024-12-15T23:15:19Z INFO  avocado::data] Parsing and preparing df
[2024-12-15T23:15:19Z INFO  avocado::data] long_df has shape: (7_605, 3)
    ┌────────────┬─────────────────────┬───────┐
    │ date       ┆ region              ┆ price │
    │ ---        ┆ ---                 ┆ ---   │
    │ date       ┆ str                 ┆ f64   │
    ╞════════════╪═════════════════════╪═══════╡
    │ 2015-01-04 ┆ Albany              ┆ 1.22  │
    │ 2015-01-04 ┆ Atlanta             ┆ 1.0   │
    │ 2015-01-04 ┆ BaltimoreWashington ┆ 1.08  │
    │ 2015-01-04 ┆ Boise               ┆ 1.01  │
    │ 2015-01-04 ┆ Boston              ┆ 1.02  │
    │ …          ┆ …                   ┆ …     │
    │ 2018-03-25 ┆ Spokane             ┆ 1.15  │
    │ 2018-03-25 ┆ StLouis             ┆ 1.25  │
    │ 2018-03-25 ┆ Syracuse            ┆ 1.38  │
    │ 2018-03-25 ┆ Tampa               ┆ 1.33  │
    │ 2018-03-25 ┆ WestTexNewMexico    ┆ 0.84  │
    └────────────┴─────────────────────┴───────┘
[2024-12-15T23:15:19Z INFO  avocado::data] Converting and diffing df
[2024-12-15T23:15:19Z INFO  avocado::data] wide_df has shape: (169, 45)
    ┌────────┬─────────┬───────────────────┬───────┬───┬─────────┬──────────┬───────┬──────────────────┐
    │ Albany ┆ Atlanta ┆ BaltimoreWashingt ┆ Boise ┆ … ┆ StLouis ┆ Syracuse ┆ Tampa ┆ WestTexNewMexico │
    │ ---    ┆ ---     ┆ on                ┆ ---   ┆   ┆ ---     ┆ ---      ┆ ---   ┆ ---              │
    │ f64    ┆ f64     ┆ ---               ┆ f64   ┆   ┆ f64     ┆ f64      ┆ f64   ┆ f64              │
    │        ┆         ┆ f64               ┆       ┆   ┆         ┆          ┆       ┆                  │
    ╞════════╪═════════╪═══════════════════╪═══════╪═══╪═════════╪══════════╪═══════╪══════════════════╡
    │ 0.0    ┆ 0.0     ┆ 0.0               ┆ 0.0   ┆ … ┆ 0.0     ┆ 0.0      ┆ 0.0   ┆ 0.0              │
    │ 0.02   ┆ 0.11    ┆ 0.09              ┆ 0.17  ┆ … ┆ 0.14    ┆ 0.12     ┆ 0.27  ┆ 0.17             │
    │ -0.07  ┆ 0.0     ┆ 0.06              ┆ -0.1  ┆ … ┆ -0.01   ┆ -0.05    ┆ 0.03  ┆ -0.12            │
    │ -0.11  ┆ -0.01   ┆ -0.03             ┆ -0.05 ┆ … ┆ -0.11   ┆ -0.03    ┆ -0.06 ┆ 0.0              │
    │ -0.07  ┆ -0.14   ┆ -0.14             ┆ -0.12 ┆ … ┆ -0.07   ┆ -0.11    ┆ -0.29 ┆ -0.09            │
    │ …      ┆ …       ┆ …                 ┆ …     ┆ … ┆ …       ┆ …        ┆ …     ┆ …                │
    │ -0.15  ┆ 0.02    ┆ -0.05             ┆ -0.16 ┆ … ┆ 0.07    ┆ -0.06    ┆ -0.03 ┆ -0.01            │
    │ -0.2   ┆ 0.02    ┆ -0.12             ┆ 0.13  ┆ … ┆ 0.03    ┆ -0.14    ┆ -0.16 ┆ 0.0              │
    │ 0.04   ┆ -0.12   ┆ 0.11              ┆ 0.04  ┆ … ┆ -0.3    ┆ 0.02     ┆ 0.02  ┆ 0.06             │
    │ 0.23   ┆ -0.01   ┆ -0.2              ┆ -0.23 ┆ … ┆ 0.05    ┆ 0.08     ┆ -0.03 ┆ -0.06            │
    │ 0.22   ┆ 0.09    ┆ 0.07              ┆ 0.25  ┆ … ┆ 0.18    ┆ 0.18     ┆ 0.2   ┆ -0.04            │
    └────────┴─────────┴───────────────────┴───────┴───┴─────────┴──────────┴───────┴──────────────────┘
[2024-12-15T23:15:19Z INFO  avocado::data] Calculating correlation matrix
[2024-12-15T23:15:19Z INFO  avocado::data] corr_df has shape: (1, 2_025)
    ┌───────────┬───────────┬───────────┬───────────┬───┬───────────┬───────────┬───────────┬──────────┐
    │ Albany,   ┆ Albany,   ┆ Albany,   ┆ Albany,   ┆ … ┆ WestTexNe ┆ WestTexNe ┆ WestTexNe ┆ WestTexN │
    │ Albany    ┆ Atlanta   ┆ Baltimore ┆ Boise     ┆   ┆ wMexico,  ┆ wMexico,  ┆ wMexico,  ┆ ewMexico │
    │ ---       ┆ ---       ┆ Washingto ┆ ---       ┆   ┆ StLouis   ┆ Syracuse  ┆ Tampa     ┆ , WestTe │
    │ f64       ┆ f64       ┆ n         ┆ f64       ┆   ┆ ---       ┆ ---       ┆ ---       ┆ xNewMe…  │
    │           ┆           ┆ ---       ┆           ┆   ┆ f64       ┆ f64       ┆ f64       ┆ ---      │
    │           ┆           ┆ f64       ┆           ┆   ┆           ┆           ┆           ┆ f64      │
    ╞═══════════╪═══════════╪═══════════╪═══════════╪═══╪═══════════╪═══════════╪═══════════╪══════════╡
    │ 1.0       ┆ 0.010539  ┆ 0.026495  ┆ 0.057347  ┆ … ┆ 0.135924  ┆ 0.179147  ┆ 0.159363  ┆ 1.0      │
    └───────────┴───────────┴───────────┴───────────┴───┴───────────┴───────────┴───────────┴──────────┘
[2024-12-15T23:15:19Z INFO  avocado::graph] Making graph
[2024-12-15T23:15:19Z INFO  avocado::graph] graph has 45 nodes and 2025 edges
[2024-12-15T23:15:19Z INFO  avocado::graph] Finding mst
[2024-12-15T23:15:19Z INFO  avocado::graph] mst has 45 nodes and 44 edges
[2024-12-15T23:15:19Z INFO  avocado] Describing and exporting mst
```
