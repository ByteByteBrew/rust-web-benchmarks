# Web Framework Benchmark Results

## Test Configuration

- Date: 2025-01-16 17:26:43
## Hardware Information
- CPU: Intel(R) Xeon(R) CPU E5-26xx v4 (2 cores)
- Memory: 3787 MB

# Performance Rankings (Requests/sec)

## Concurrency: 64

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | ntex          59658 | ntex          60385 | may_minihttp     61826 |
|    2 | xitca-web     51738 | may_minihttp     58393 | ntex          58767 |
|    3 | ohkami        50496 | xitca-web     55669 | ohkami        57436 |
|    4 | actix-web     49928 | ohkami        50872 | actix-web     53719 |
|    5 | may_minihttp     49704 | actix-web     48482 | xitca-web     53550 |
|    6 | thruster      48455 | thruster      46070 | argan         49155 |
|    7 | warp          44560 | argan         45836 | trillium      48533 |
|    8 | trillium      42959 | warp          45462 | thruster      48079 |
|    9 | argan         41997 | trillium      44759 | warp          46890 |
|   10 | graphul       41441 | iron          40781 | axum          44813 |
|   11 | axum          40263 | graphul       40755 | iron          43883 |
|   12 | viz           39862 | astra         39539 | saphir        42416 |
|   13 | gotham        39159 | viz           39506 | poem          41502 |
|   14 | astra         38804 | axum          39497 | gotham        41256 |
|   15 | poem          38397 | poem          39223 | graphul       41115 |
|   16 | saphir        37912 | gotham        38946 | viz           38823 |
|   17 | salvo         36846 | salvo         35816 | astra         37959 |
|   18 | iron          35250 | saphir        35125 | salvo         37876 |
|   19 | silent        28516 | silent        29832 | silent        30674 |
|   20 | nickel        26238 | nickel        28152 | nickel        27423 |
|   21 | rocket        24490 | rocket        23947 | rocket        25964 |
|   22 | oxidy         11602 | oxidy         11475 | oxidy         12284 |
|   23 | rouille        6360 | rouille        6526 | rouille        6572 |
|   24 | summer-boot      1410 | summer-boot      1418 | summer-boot      1414 |
|   25 | tide           1407 | tide           1409 | tide           1413 |

## Concurrency: 256

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | may_minihttp     50636 | ntex          52307 | ntex          57545 |
|    2 | ntex          49030 | xitca-web     49496 | actix-web     52701 |
|    3 | actix-web     45542 | may_minihttp     47787 | xitca-web     50079 |
|    4 | astra         45431 | thruster      46372 | trillium      47749 |
|    5 | xitca-web     45217 | ohkami        46205 | thruster      47261 |
|    6 | trillium      45014 | actix-web     45359 | may_minihttp     47074 |
|    7 | thruster      43751 | trillium      42708 | ohkami        44887 |
|    8 | ohkami        41400 | argan         42316 | argan         43437 |
|    9 | argan         40177 | astra         39911 | warp          43412 |
|   10 | warp          40049 | warp          38895 | axum          40314 |
|   11 | saphir        36570 | axum          38128 | astra         39613 |
|   12 | graphul       35275 | gotham        36213 | viz           39131 |
|   13 | axum          35247 | poem          34907 | gotham        38044 |
|   14 | gotham        35145 | viz           33332 | graphul       37033 |
|   15 | viz           34486 | graphul       33095 | saphir        36802 |
|   16 | poem          32824 | salvo         30959 | salvo         34588 |
|   17 | salvo         29817 | saphir        30837 | poem          34465 |
|   18 | iron          27124 | iron          27686 | iron          32409 |
|   19 | nickel        26855 | nickel        26611 | nickel        28483 |
|   20 | silent        26725 | silent        23523 | silent        25466 |
|   21 | rocket        21314 | rocket        21412 | rocket        19266 |
|   22 | oxidy         11264 | oxidy         11030 | oxidy         12225 |
|   23 | rouille        5641 | rouille        5850 | rouille        5919 |
|   24 | summer-boot      5464 | summer-boot      5467 | tide           5523 |
|   25 | tide           5456 | tide           5292 | summer-boot      5452 |

## Concurrency: 512

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | trillium      48046 | ntex          50330 | may_minihttp     50068 |
|    2 | ohkami        45566 | may_minihttp     48135 | ntex          49473 |
|    3 | may_minihttp     44803 | trillium      44770 | actix-web     49009 |
|    4 | ntex          43801 | actix-web     44267 | trillium      48447 |
|    5 | thruster      42115 | xitca-web     43476 | ohkami        45856 |
|    6 | actix-web     42110 | astra         43440 | thruster      44476 |
|    7 | astra         41951 | ohkami        42737 | xitca-web     42295 |
|    8 | xitca-web     41820 | thruster      41401 | warp          41989 |
|    9 | argan         36743 | argan         39845 | argan         41299 |
|   10 | warp          36245 | warp          37864 | astra         38341 |
|   11 | saphir        34991 | nickel        36983 | gotham        37062 |
|   12 | axum          33391 | axum          34840 | axum          36466 |
|   13 | gotham        32690 | gotham        34690 | graphul       35059 |
|   14 | viz           31593 | saphir        31763 | saphir        34750 |
|   15 | graphul       30864 | graphul       31760 | viz           34495 |
|   16 | poem          29264 | poem          31559 | poem          32153 |
|   17 | iron          27855 | viz           31363 | iron          30284 |
|   18 | salvo         27615 | salvo         31107 | salvo         29903 |
|   19 | nickel        26982 | iron          29649 | nickel        28762 |
|   20 | silent        25013 | silent        26064 | silent        23863 |
|   21 | rocket        22089 | rocket        22019 | rocket        21879 |
|   22 | oxidy         10729 | oxidy         11415 | oxidy         11660 |
|   23 | tide          10332 | summer-boot     10180 | summer-boot     10274 |
|   24 | summer-boot      9967 | tide          10085 | tide          10122 |
|   25 | rouille        5717 | rouille        5341 | rouille        5531 |
