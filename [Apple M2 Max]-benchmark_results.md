# Web Framework Benchmark Results

## Test Configuration

- Date: 2024-11-21 12:52:50
## Hardware Information
- CPU:  (8 cores)
- Memory: 7997 MB

# Performance Rankings (Requests/sec)

## Concurrency: 64

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | xitca-web    911262 | actix-web    942061 | xitca-web   1124681 |
|    2 | actix-web    910204 | xitca-web    914010 | actix-web    993618 |
|    3 | ntex         870524 | ntex         908683 | ntex         937368 |
|    4 | iron         637241 | iron         658239 | iron         656916 |
|    5 | warp         449311 | ohkami       456884 | ohkami       482897 |
|    6 | poem         432523 | warp         451100 | argan        463859 |
|    7 | argan        427589 | argan        447243 | warp         462737 |
|    8 | viz          423866 | poem         434126 | graphul      445722 |
|    9 | ohkami       423428 | graphul      431737 | poem         443083 |
|   10 | gotham       416177 | salvo        429661 | axum         442043 |
|   11 | axum         402000 | gotham       413673 | salvo        429551 |
|   12 | graphul      386590 | viz          403554 | gotham       401273 |
|   13 | salvo        381610 | axum         389268 | viz          398849 |
|   14 | rocket       359426 | rocket       362179 | rocket       366280 |
|   15 | silent       323715 | nickel       332282 | silent       320312 |
|   16 | nickel       275238 | silent       326281 | nickel       310257 |
|   17 | oxidy        123037 | oxidy        119540 | oxidy        143898 |
|   18 | summer-boot      1488 | summer-boot      1490 | tide           1479 |
|   19 | tide           1484 | tide           1489 | summer-boot      1473 |
|   20 | may_minihttp         0 | may_minihttp         0 | may_minihttp         0 |

## Concurrency: 256

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | actix-web   1036612 | xitca-web   1099951 | xitca-web   1124880 |
|    2 | xitca-web   1019971 | actix-web   1075871 | actix-web   1119460 |
|    3 | ntex        1002255 | ntex        1056666 | ntex        1097828 |
|    4 | warp         756051 | warp         798059 | warp         817246 |
|    5 | ohkami       726474 | ohkami       750747 | ohkami       778708 |
|    6 | argan        705991 | argan        727280 | argan        757026 |
|    7 | viz          698503 | viz          706569 | axum         740161 |
|    8 | axum         677321 | axum         687392 | viz          738762 |
|    9 | graphul      672355 | gotham       673427 | graphul      723347 |
|   10 | gotham       652375 | graphul      672638 | gotham       696857 |
|   11 | poem         622204 | poem         647009 | poem         660226 |
|   12 | salvo        611485 | salvo        638853 | salvo        654169 |
|   13 | iron         457794 | iron         506994 | iron         535338 |
|   14 | rocket       426688 | rocket       429145 | rocket       442220 |
|   15 | silent       339343 | silent       346475 | silent       353896 |
|   16 | nickel       291808 | nickel       267087 | nickel       322100 |
|   17 | oxidy        136979 | oxidy        124167 | oxidy        154779 |
|   18 | tide           6086 | summer-boot      6013 | summer-boot      6020 |
|   19 | summer-boot      5845 | tide           5969 | tide           5847 |
|   20 | may_minihttp         0 | may_minihttp         0 | may_minihttp         0 |

## Concurrency: 512

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | actix-web   1071411 | xitca-web   1158138 | actix-web   1187841 |
|    2 | xitca-web   1056864 | actix-web   1131099 | ntex        1139000 |
|    3 | ntex        1044291 | ntex        1100905 | xitca-web   1095352 |
|    4 | ohkami       855822 | ohkami       887242 | warp         915966 |
|    5 | warp         802898 | warp         856925 | ohkami       909515 |
|    6 | argan        769444 | argan        815357 | argan        848497 |
|    7 | viz          736220 | viz          731812 | viz          809896 |
|    8 | gotham       699504 | axum         716129 | axum         797569 |
|    9 | axum         691829 | gotham       715492 | gotham       757865 |
|   10 | graphul      686720 | graphul      700903 | graphul      755872 |
|   11 | poem         646975 | poem         663343 | poem         690854 |
|   12 | salvo        634722 | salvo        662863 | salvo        689060 |
|   13 | rocket       425881 | iron         508196 | iron         499835 |
|   14 | iron         342563 | rocket       431574 | rocket       449218 |
|   15 | silent       341394 | nickel       386798 | silent       350584 |
|   16 | nickel       282135 | silent       350189 | nickel       228239 |
|   17 | oxidy        145664 | oxidy        125625 | oxidy        160736 |
|   18 | tide          12205 | summer-boot     12022 | tide          11742 |
|   19 | summer-boot     11921 | tide          11859 | summer-boot     11557 |
|   20 | may_minihttp         0 | may_minihttp         0 | may_minihttp         0 |
