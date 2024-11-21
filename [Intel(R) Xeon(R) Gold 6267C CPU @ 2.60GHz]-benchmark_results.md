# Web Framework Benchmark Results

## Test Configuration

- Date: 2024-11-21 11:32:32
## Hardware Information
- CPU: Intel(R) Xeon(R) Gold 6267C CPU @ 2.60GHz (8 cores)
- Memory: 32095 MB

# Performance Rankings (Requests/sec)

## Concurrency: 64

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | may_minihttp    230441 | may_minihttp    232805 | may_minihttp    230386 |
|    2 | ntex         207794 | actix-web    208055 | actix-web    216473 |
|    3 | actix-web    206382 | ntex         206446 | ntex         216280 |
|    4 | xitca-web    196860 | xitca-web    196732 | xitca-web    203603 |
|    5 | ohkami       184587 | ohkami       185890 | ohkami       189576 |
|    6 | warp         171091 | argan        172979 | warp         178682 |
|    7 | argan        169391 | warp         172234 | argan        176879 |
|    8 | iron         168313 | iron         170368 | iron         175690 |
|    9 | graphul      162648 | viz          162915 | viz          169247 |
|   10 | viz          161951 | poem         162175 | axum         168055 |
|   11 | poem         159663 | graphul      159913 | graphul      165882 |
|   12 | axum         158656 | axum         159657 | poem         165264 |
|   13 | salvo        156723 | salvo        159249 | salvo        162926 |
|   14 | gotham       153542 | gotham       154693 | gotham       160993 |
|   15 | silent       138535 | silent       140153 | silent       143237 |
|   16 | rocket       124137 | nickel       126496 | rocket       129684 |
|   17 | nickel       106447 | rocket       124938 | nickel        94646 |
|   18 | oxidy         40998 | oxidy         40968 | oxidy         42575 |
|   19 | tide           1432 | summer-boot      1433 | summer-boot      1433 |
|   20 | summer-boot      1430 | tide           1431 | tide           1429 |

## Concurrency: 256

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | may_minihttp    243865 | may_minihttp    249432 | may_minihttp    248430 |
|    2 | actix-web    235244 | ntex         239212 | actix-web    244105 |
|    3 | ntex         234393 | actix-web    235008 | ntex         243732 |
|    4 | xitca-web    217930 | xitca-web    221929 | ohkami       226566 |
|    5 | ohkami       216349 | ohkami       220663 | xitca-web    224784 |
|    6 | warp         200571 | warp         203873 | warp         208411 |
|    7 | argan        199497 | argan        202973 | argan        206387 |
|    8 | graphul      189697 | viz          191250 | viz          195405 |
|    9 | viz          188511 | graphul      189313 | axum         195309 |
|   10 | axum         188146 | poem         188633 | graphul      193633 |
|   11 | poem         185711 | axum         188222 | poem         193265 |
|   12 | salvo        183663 | salvo        185424 | salvo        189338 |
|   13 | gotham       180919 | gotham       181604 | gotham       187320 |
|   14 | iron         156636 | iron         157238 | iron         168082 |
|   15 | silent       142267 | silent       145955 | silent       147530 |
|   16 | rocket       132267 | rocket       131186 | rocket       140724 |
|   17 | nickel       131537 | nickel       107071 | nickel       139899 |
|   18 | oxidy         45393 | oxidy         44159 | oxidy         46884 |
|   19 | tide           5693 | tide           5664 | summer-boot      5625 |
|   20 | summer-boot      5612 | summer-boot      5634 | tide           5586 |

## Concurrency: 512

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | may_minihttp    248621 | may_minihttp    250841 | may_minihttp    256529 |
|    2 | ntex         239921 | ntex         243422 | ntex         251957 |
|    3 | actix-web    237032 | actix-web    240636 | actix-web    249879 |
|    4 | ohkami       230802 | ohkami       235591 | ohkami       239851 |
|    5 | xitca-web    222188 | xitca-web    224082 | xitca-web    229848 |
|    6 | warp         212160 | warp         217732 | warp         224333 |
|    7 | argan        211170 | argan        214188 | argan        220622 |
|    8 | viz          204283 | viz          203275 | axum         210671 |
|    9 | graphul      200792 | graphul      200314 | viz          209948 |
|   10 | axum         199595 | axum         200264 | graphul      209386 |
|   11 | poem         197581 | poem         198918 | poem         203340 |
|   12 | salvo        192178 | salvo        193676 | salvo        201389 |
|   13 | gotham       191162 | gotham       191115 | gotham       199280 |
|   14 | iron         164983 | iron         159653 | iron         161806 |
|   15 | silent       142734 | silent       139990 | silent       149844 |
|   16 | rocket       130839 | rocket       131823 | rocket       138386 |
|   17 | nickel       114627 | nickel       108087 | nickel       109024 |
|   18 | oxidy         44784 | oxidy         44334 | oxidy         47177 |
|   19 | tide          11152 | summer-boot     11123 | summer-boot     11195 |
|   20 | summer-boot     11056 | tide          11039 | tide          11081 |
