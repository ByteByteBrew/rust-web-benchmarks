# Web Framework Benchmark Results

## Test Configuration

- Date: 2024-12-06 18:12:39
## Hardware Information
- CPU: Intel(R) Xeon(R) Gold 6267C CPU @ 2.60GHz (8 cores)
- Memory: 32095 MB

# Performance Rankings (Requests/sec)

## Concurrency: 64

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | may_minihttp    193245 | may_minihttp    187917 | actix-web    188033 |
|    2 | ntex         183127 | actix-web    186755 | may_minihttp    187050 |
|    3 | actix-web    182694 | ntex         186691 | ntex         186105 |
|    4 | xitca-web    177393 | xitca-web    176142 | xitca-web    180461 |
|    5 | ohkami       160092 | ohkami       165985 | ohkami       167675 |
|    6 | argan        156252 | warp         157016 | argan        161157 |
|    7 | warp         154844 | argan        154342 | warp         158436 |
|    8 | thruster     153714 | thruster     154186 | viz          155978 |
|    9 | viz          151390 | viz          151650 | thruster     155680 |
|   10 | axum         149600 | axum         146765 | graphul      152396 |
|   11 | graphul      147194 | poem         146643 | axum         151349 |
|   12 | poem         144955 | graphul      146324 | poem         149203 |
|   13 | salvo        143725 | salvo        145493 | salvo        146902 |
|   14 | gotham       139161 | gotham       145071 | iron         144564 |
|   15 | iron         138212 | iron         137659 | gotham       144215 |
|   16 | saphir       135591 | astra        131375 | silent       135624 |
|   17 | silent       132713 | silent       129940 | astra        133829 |
|   18 | trillium     131259 | trillium     127703 | trillium     131166 |
|   19 | astra        130689 | saphir       125024 | rocket       120994 |
|   20 | rocket       117320 | rocket       120798 | saphir       118656 |
|   21 | nickel       106213 | nickel        86562 | nickel        97561 |
|   22 | oxidy         36930 | oxidy         37870 | oxidy         38198 |
|   23 | rouille       19857 | rouille       20601 | rouille       19938 |
|   24 | tide           1416 | summer-boot      1417 | tide           1418 |
|   25 | summer-boot      1414 | tide           1413 | summer-boot      1417 |

## Concurrency: 256

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | ntex         207661 | ntex         210939 | may_minihttp    211404 |
|    2 | may_minihttp    206480 | may_minihttp    207261 | ntex         210688 |
|    3 | actix-web    202233 | actix-web    205032 | actix-web    207586 |
|    4 | ohkami       197814 | ohkami       200843 | ohkami       204949 |
|    5 | xitca-web    195745 | warp         190524 | xitca-web    196719 |
|    6 | thruster     186835 | argan        190168 | argan        191826 |
|    7 | warp         184386 | xitca-web    189985 | thruster     190222 |
|    8 | argan        181915 | thruster     187201 | warp         189853 |
|    9 | viz          178664 | viz          178877 | axum         184126 |
|   10 | axum         174348 | poem         176253 | graphul      182569 |
|   11 | graphul      173025 | axum         173769 | viz          182001 |
|   12 | salvo        170984 | salvo        172270 | poem         175291 |
|   13 | poem         166881 | graphul      172020 | salvo        172654 |
|   14 | gotham       163700 | gotham       170001 | gotham       170567 |
|   15 | saphir       163491 | trillium     152300 | trillium     163640 |
|   16 | trillium     156005 | saphir       149643 | saphir       157923 |
|   17 | silent       145281 | silent       146275 | silent       148188 |
|   18 | rocket       129375 | iron         134273 | rocket       137165 |
|   19 | astra        126971 | astra        133334 | astra        134715 |
|   20 | iron         113454 | rocket       131976 | iron         105133 |
|   21 | nickel        51442 | nickel        52067 | nickel        51649 |
|   22 | oxidy         41880 | oxidy         40804 | oxidy         42043 |
|   23 | rouille       18448 | rouille       19092 | rouille       18823 |
|   24 | summer-boot      5642 | summer-boot      5640 | summer-boot      5656 |
|   25 | tide           5595 | tide           5635 | tide           5624 |

## Concurrency: 512

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | ntex         212270 | ntex         213270 | ohkami       218507 |
|    2 | ohkami       212166 | ohkami       209871 | ntex         217774 |
|    3 | may_minihttp    205239 | may_minihttp    209534 | actix-web    215234 |
|    4 | actix-web    204933 | actix-web    207460 | may_minihttp    214817 |
|    5 | thruster     197561 | warp         200273 | thruster     202844 |
|    6 | xitca-web    194884 | argan        199846 | xitca-web    201887 |
|    7 | warp         192102 | thruster     196686 | argan        201774 |
|    8 | argan        190562 | xitca-web    189096 | warp         199563 |
|    9 | viz          184509 | viz          187893 | viz          194110 |
|   10 | graphul      179338 | axum         184631 | axum         193838 |
|   11 | salvo        177955 | poem         181264 | graphul      190589 |
|   12 | axum         175592 | graphul      177892 | poem         187225 |
|   13 | poem         175013 | salvo        177873 | gotham       182089 |
|   14 | gotham       174257 | gotham       171874 | salvo        180729 |
|   15 | saphir       172197 | saphir       159292 | saphir       171410 |
|   16 | trillium     157248 | trillium     159097 | trillium     161796 |
|   17 | silent       142405 | silent       143602 | silent       145976 |
|   18 | rocket       130218 | astra        129895 | rocket       141399 |
|   19 | nickel       129756 | rocket       128450 | astra        132060 |
|   20 | astra        127740 | iron          55158 | iron          55066 |
|   21 | iron          55208 | nickel        51766 | nickel        51374 |
|   22 | oxidy         42405 | oxidy         40395 | oxidy         43142 |
|   23 | rouille       18372 | rouille       18627 | rouille       18434 |
|   24 | tide          11004 | summer-boot     10969 | summer-boot     11022 |
|   25 | summer-boot     10880 | tide          10675 | tide          10996 |
