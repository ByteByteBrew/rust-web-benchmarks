# Web Framework Benchmark Results

## Test Configuration

- Date: 2024-11-18 17:25:21
## Hardware Information
- CPU:  (8 cores)
- Memory: 7997 MB

# Performance Rankings (Requests/sec)

## Concurrency: 64

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | xitca-web    962713 | actix-web    939343 | xitca-web   1093415 |
|    2 | actix-web    915922 | xitca-web    924102 | actix-web    999313 |
|    3 | ntex         865544 | ntex         901776 | ntex         947246 |
|    4 | iron         631430 | iron         650744 | iron         658898 |
|    5 | axum         426534 | argan        443087 | warp         475788 |
|    6 | argan        422944 | axum         432817 | argan        470760 |
|    7 | warp         420651 | ohkami       432426 | ohkami       466066 |
|    8 | ohkami       420268 | warp         418080 | axum         463733 |
|    9 | salvo        407382 | viz          412299 | graphul      433299 |
|   10 | viz          395719 | graphul      404085 | viz          412143 |
|   11 | graphul      379844 | salvo        403665 | salvo        401320 |
|   12 | poem         348722 | poem         378460 | gotham       372863 |
|   13 | gotham       337329 | gotham       365656 | poem         370682 |
|   14 | rocket       336875 | rocket       329467 | rocket       328471 |
|   15 | silent       315805 | silent       323615 | silent       315361 |
|   16 | nickel       260642 | nickel       307358 | nickel       287080 |
|   17 | oxidy        121638 | oxidy        112474 | oxidy        141418 |
|   18 | summer-boot      1509 | summer-boot      1509 | tide           1512 |
|   19 | tide           1509 | tide           1508 | summer-boot      1509 |
|   20 | may_minihttp         0 | may_minihttp         0 | may_minihttp         0 |

## Concurrency: 256

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | xitca-web   1106228 | xitca-web   1160496 | actix-web   1144994 |
|    2 | actix-web   1028207 | actix-web   1084468 | ntex        1102371 |
|    3 | ntex        1003183 | ntex        1045983 | xitca-web   1072389 |
|    4 | warp         752690 | warp         791954 | warp         835399 |
|    5 | ohkami       732675 | ohkami       742413 | axum         753077 |
|    6 | argan        698231 | argan        730197 | ohkami       751618 |
|    7 | viz          691101 | viz          707083 | viz          744170 |
|    8 | axum         687735 | axum         694673 | argan        738959 |
|    9 | graphul      665760 | graphul      672331 | graphul      720998 |
|   10 | gotham       639251 | gotham       668762 | gotham       687285 |
|   11 | poem         618891 | poem         624172 | poem         661853 |
|   12 | salvo        603729 | salvo        622276 | salvo        658639 |
|   13 | iron         481416 | iron         537430 | iron         520574 |
|   14 | rocket       420805 | rocket       433511 | rocket       441715 |
|   15 | silent       342652 | nickel       356228 | silent       355832 |
|   16 | nickel       255005 | silent       341748 | nickel       311157 |
|   17 | oxidy        137744 | oxidy        120750 | oxidy        151485 |
|   18 | summer-boot      6143 | tide           6085 | may_minihttp     27311 |
|   19 | tide           6113 | summer-boot      6015 | summer-boot      6135 |
|   20 | may_minihttp         0 | may_minihttp         0 | tide           5916 |

## Concurrency: 512

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | xitca-web   1109367 | xitca-web   1134400 | actix-web   1196295 |
|    2 | actix-web   1099765 | actix-web   1131380 | ntex        1154856 |
|    3 | ntex        1052256 | ntex        1108013 | xitca-web   1111670 |
|    4 | ohkami       860038 | ohkami       887600 | ohkami       910613 |
|    5 | warp         799571 | warp         846037 | warp         883087 |
|    6 | argan        788245 | argan        810554 | argan        836124 |
|    7 | viz          720182 | viz          735940 | axum         810314 |
|    8 | axum         706870 | axum         730946 | viz          806128 |
|    9 | graphul      689735 | gotham       706839 | graphul      761644 |
|   10 | gotham       685345 | graphul      701043 | gotham       760341 |
|   11 | salvo        626913 | salvo        653168 | poem         672786 |
|   12 | poem         623350 | poem         638677 | salvo        671774 |
|   13 | rocket       430795 | iron         491980 | iron         592881 |
|   14 | silent       332585 | rocket       431780 | rocket       455345 |
|   15 | iron         330920 | silent       343235 | silent       347606 |
|   16 | nickel       322881 | nickel       283725 | nickel       286452 |
|   17 | oxidy        139763 | oxidy        121810 | oxidy        156010 |
|   18 | summer-boot     12074 | summer-boot     12119 | tide          12296 |
|   19 | tide          11611 | tide          12065 | summer-boot     11849 |
|   20 | may_minihttp         0 | may_minihttp         0 | may_minihttp         0 |
