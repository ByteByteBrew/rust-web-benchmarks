# Web Framework Benchmark Results

## Test Configuration

- Date: 2024-11-19 00:21:13
## Hardware Information
- CPU: AMD Ryzen 9 7950X 16-Core Processor (32 cores)
- Memory: 31699 MB

# Performance Rankings (Requests/sec)

## Concurrency: 64

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | xitca-web    459091 | xitca-web    456673 | xitca-web    439401 |
|    2 | ohkami       432321 | ohkami       432587 | ohkami       438356 |
|    3 | warp         417392 | warp         421326 | warp         426509 |
|    4 | viz          414102 | ntex         414311 | axum         421442 |
|    5 | argan        413522 | argan        414154 | argan        418143 |
|    6 | axum         413350 | axum         413347 | graphul      417146 |
|    7 | ntex         412673 | graphul      413166 | viz          416299 |
|    8 | graphul      412048 | viz          412767 | ntex         416159 |
|    9 | silent       411035 | silent       412665 | poem         414094 |
|   10 | salvo        410552 | actix-web    412439 | silent       413196 |
|   11 | poem         410142 | poem         410569 | may_minihttp    412791 |
|   12 | gotham       408922 | salvo        410382 | salvo        412737 |
|   13 | may_minihttp    408633 | may_minihttp    408227 | actix-web    412219 |
|   14 | actix-web    407539 | gotham       407349 | gotham       411504 |
|   15 | rocket       399762 | rocket       399759 | rocket       400977 |
|   16 | iron         352477 | iron         354057 | iron         354540 |
|   17 | nickel       234190 | nickel       272433 | nickel       282776 |
|   18 | oxidy        103600 | oxidy         88417 | oxidy        105490 |
|   19 | tide           1222 | tide           1222 | tide           1210 |
|   20 | summer-boot      1218 | summer-boot      1218 | summer-boot      1208 |

## Concurrency: 256

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | ohkami       978211 | ohkami       984163 | ohkami       998242 |
|    2 | warp         911489 | warp         927888 | warp         942132 |
|    3 | argan        911334 | argan        913411 | argan        930530 |
|    4 | viz          895083 | viz          890184 | axum         912419 |
|    5 | axum         884553 | axum         883288 | viz          907367 |
|    6 | graphul      869705 | graphul      867167 | graphul      890766 |
|    7 | poem         835919 | poem         843882 | gotham       877678 |
|    8 | gotham       833955 | gotham       842398 | poem         869262 |
|    9 | salvo        813634 | salvo        823213 | salvo        851007 |
|   10 | silent       683692 | silent       682804 | silent       672098 |
|   11 | xitca-web    514847 | xitca-web    579278 | xitca-web    539936 |
|   12 | ntex         499740 | ntex         497764 | ntex         499360 |
|   13 | actix-web    485050 | actix-web    490412 | actix-web    487708 |
|   14 | may_minihttp    471101 | may_minihttp    477110 | may_minihttp    481022 |
|   15 | iron         359840 | rocket       452692 | rocket       463547 |
|   16 | rocket       345816 | iron         361645 | iron         361664 |
|   17 | nickel       182939 | nickel       184467 | nickel       232490 |
|   18 | oxidy        105448 | oxidy         87159 | oxidy        104394 |
|   19 | tide           4697 | tide           4695 | tide           4690 |
|   20 | summer-boot      4671 | summer-boot      4689 | summer-boot      4682 |

## Concurrency: 512

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | ohkami      1364845 | ohkami      1390381 | ohkami      1412542 |
|    2 | argan       1162757 | argan       1167026 | argan       1228890 |
|    3 | warp        1090592 | warp        1131844 | warp        1162018 |
|    4 | viz         1089990 | axum        1092994 | viz         1112985 |
|    5 | axum        1075189 | viz         1085604 | axum        1109792 |
|    6 | salvo       1062535 | graphul     1068779 | graphul     1091327 |
|    7 | graphul     1061855 | salvo       1067261 | gotham      1084366 |
|    8 | gotham      1060638 | gotham      1063838 | poem        1071477 |
|    9 | poem        1058378 | poem        1059716 | salvo       1063355 |
|   10 | silent       713878 | silent       731659 | silent       703489 |
|   11 | ntex         581735 | ntex         579921 | ntex         579733 |
|   12 | xitca-web    576459 | xitca-web    576565 | xitca-web    575593 |
|   13 | actix-web    559796 | may_minihttp    548761 | may_minihttp    564925 |
|   14 | may_minihttp    551552 | actix-web    544949 | actix-web    556053 |
|   15 | iron         356414 | iron         358523 | rocket       392729 |
|   16 | rocket       315558 | rocket       310987 | iron         358942 |
|   17 | nickel       183065 | nickel       184953 | nickel       164144 |
|   18 | oxidy        105447 | oxidy         86984 | oxidy        103407 |
|   19 | tide           9351 | tide           9455 | tide           9439 |
|   20 | summer-boot      9349 | summer-boot      9413 | summer-boot      9355 |
