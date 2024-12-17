# Web Framework Benchmark Results

## Test Configuration

- Date: 2024-12-17 18:23:45
## Hardware Information
- CPU: Intel(R) Xeon(R) CPU E5-26xx v4 (2 cores)
- Memory: 3787 MB

# Performance Rankings (Requests/sec)

## Concurrency: 64

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | may_minihttp     55825 | may_minihttp     57927 | may_minihttp     62715 |
|    2 | ohkami        52456 | ntex          53585 | ntex          58031 |
|    3 | ntex          51500 | xitca-web     51845 | ohkami        56022 |
|    4 | xitca-web     49042 | ohkami        50691 | actix-web     51954 |
|    5 | actix-web     47651 | actix-web     50052 | xitca-web     50400 |
|    6 | thruster      46120 | thruster      46655 | warp          47527 |
|    7 | warp          45171 | warp          44485 | thruster      45861 |
|    8 | argan         43344 | trillium      42670 | astra         45187 |
|    9 | graphul       43008 | astra         41868 | trillium      43943 |
|   10 | astra         40341 | iron          41782 | argan         43877 |
|   11 | trillium      40229 | graphul       38725 | graphul       42720 |
|   12 | saphir        39482 | argan         38712 | axum          40373 |
|   13 | viz           39259 | viz           38510 | viz           39882 |
|   14 | axum          39151 | axum          37667 | saphir        39790 |
|   15 | gotham        37811 | salvo         36781 | gotham        39565 |
|   16 | poem          37567 | gotham        36458 | poem          39448 |
|   17 | salvo         35302 | poem          36430 | iron          38023 |
|   18 | iron          35057 | saphir        35241 | salvo         37905 |
|   19 | silent        29070 | silent        29944 | silent        31381 |
|   20 | nickel        27841 | nickel        28416 | nickel        27779 |
|   21 | rocket        25957 | rocket        24301 | rocket        26632 |
|   22 | oxidy         11629 | oxidy         12782 | oxidy         12566 |
|   23 | rouille        6335 | rouille        6477 | rouille        6580 |
|   24 | tide           1427 | tide           1428 | tide           1428 |
|   25 | summer-boot      1425 | summer-boot      1427 | summer-boot      1418 |

## Concurrency: 256

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | ohkami        50119 | may_minihttp     53670 | may_minihttp     54881 |
|    2 | may_minihttp     48840 | ntex          51075 | actix-web     53899 |
|    3 | ntex          48115 | xitca-web     48535 | ohkami        53479 |
|    4 | actix-web     47129 | actix-web     47351 | ntex          52014 |
|    5 | xitca-web     44323 | ohkami        45060 | xitca-web     49233 |
|    6 | warp          42358 | trillium      42336 | thruster      47104 |
|    7 | thruster      42289 | astra         41804 | warp          43550 |
|    8 | astra         40454 | thruster      40658 | astra         41652 |
|    9 | trillium      39465 | iron          39055 | trillium      40854 |
|   10 | iron          36514 | warp          38909 | axum          40474 |
|   11 | viz           36483 | argan         37263 | viz           40133 |
|   12 | graphul       36344 | axum          36147 | argan         39511 |
|   13 | argan         35705 | gotham        36090 | graphul       37971 |
|   14 | axum          35640 | poem          35431 | gotham        36294 |
|   15 | saphir        33931 | graphul       34789 | saphir        36108 |
|   16 | salvo         33674 | salvo         33759 | poem          34404 |
|   17 | gotham        32783 | viz           33606 | salvo         33324 |
|   18 | poem          32153 | saphir        32249 | iron          29612 |
|   19 | silent        28150 | silent        27701 | silent        28476 |
|   20 | nickel        27397 | nickel        27552 | nickel        27289 |
|   21 | rocket        22090 | rocket        21842 | rocket        23901 |
|   22 | oxidy         12065 | oxidy         11737 | oxidy         12909 |
|   23 | summer-boot      5584 | rouille        5942 | rouille        5829 |
|   24 | tide           5567 | tide           5541 | summer-boot      5598 |
|   25 | rouille        5402 | summer-boot      5530 | tide           5595 |

## Concurrency: 512

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | may_minihttp     49013 | may_minihttp     47864 | may_minihttp     55224 |
|    2 | ohkami        48343 | astra         45290 | ohkami        48244 |
|    3 | ntex          43792 | ohkami        45048 | ntex          46993 |
|    4 | actix-web     41437 | actix-web     43665 | trillium      46912 |
|    5 | trillium      41423 | ntex          43064 | actix-web     46827 |
|    6 | astra         38312 | xitca-web     41718 | xitca-web     45327 |
|    7 | xitca-web     37425 | warp          39676 | astra         41328 |
|    8 | thruster      36069 | thruster      37783 | thruster      41131 |
|    9 | warp          35523 | trillium      35815 | warp          39826 |
|   10 | argan         34466 | argan         34345 | argan         36928 |
|   11 | graphul       33554 | axum          33413 | axum          36740 |
|   12 | gotham        32908 | viz           33384 | gotham        35068 |
|   13 | saphir        32641 | gotham        33053 | graphul       34580 |
|   14 | axum          32173 | graphul       32424 | viz           34364 |
|   15 | viz           31360 | poem          31630 | saphir        34212 |
|   16 | poem          30519 | salvo         31113 | poem          33435 |
|   17 | salvo         29660 | saphir        30544 | iron          29789 |
|   18 | iron          28401 | iron          30439 | salvo         29719 |
|   19 | nickel        27280 | nickel        26937 | nickel        27844 |
|   20 | silent        26881 | silent        26734 | silent        25139 |
|   21 | rocket        22621 | rocket        22218 | rocket        22361 |
|   22 | oxidy         11683 | oxidy         11305 | oxidy         11557 |
|   23 | tide          10422 | tide          10618 | summer-boot     10655 |
|   24 | summer-boot     10153 | summer-boot     10316 | tide          10451 |
|   25 | rouille        5632 | rouille        5422 | rouille        5754 |
