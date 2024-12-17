# Web Framework Benchmark Results

## Test Configuration

- Date: 2024-12-17 15:46:04
## Hardware Information
- CPU: Intel(R) Xeon(R) Gold 6267C CPU @ 2.60GHz (8 cores)
- Memory: 32095 MB

# Performance Rankings (Requests/sec)

## Concurrency: 64

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | may_minihttp    180446 | may_minihttp    178489 | ntex         183214 |
|    2 | actix-web    177624 | ntex         177022 | actix-web    182856 |
|    3 | ntex         175825 | actix-web    176735 | may_minihttp    179010 |
|    4 | xitca-web    164402 | xitca-web    164309 | xitca-web    169858 |
|    5 | ohkami       153471 | ohkami       152858 | ohkami       161474 |
|    6 | thruster     147291 | warp         150396 | argan        150483 |
|    7 | viz          145760 | argan        145584 | warp         149743 |
|    8 | argan        145314 | thruster     145384 | thruster     147996 |
|    9 | graphul      144737 | axum         144204 | axum         145692 |
|   10 | warp         144272 | viz          141962 | viz          140926 |
|   11 | poem         141260 | graphul      141190 | poem         140707 |
|   12 | gotham       139625 | poem         140210 | graphul      140222 |
|   13 | axum         138594 | gotham       139276 | gotham       140011 |
|   14 | salvo        137280 | salvo        139164 | salvo        138473 |
|   15 | saphir       136763 | iron         137065 | iron         137839 |
|   16 | iron         134415 | trillium     128758 | saphir       136020 |
|   17 | astra        126059 | saphir       127927 | trillium     128865 |
|   18 | silent       122973 | astra        126971 | astra        127766 |
|   19 | trillium     121824 | silent       121633 | silent       123767 |
|   20 | rocket       115280 | rocket       116408 | rocket       121764 |
|   21 | nickel        82766 | nickel        83345 | nickel        84068 |
|   22 | oxidy         35188 | oxidy         35489 | oxidy         36867 |
|   23 | rouille       19000 | rouille       19582 | rouille       20116 |
|   24 | tide           1416 | tide           1417 | summer-boot      1418 |
|   25 | summer-boot      1415 | summer-boot      1416 | tide           1415 |

## Concurrency: 256

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | ntex         196484 | ntex         200354 | ntex         206717 |
|    2 | may_minihttp    193214 | may_minihttp    198526 | actix-web    205768 |
|    3 | actix-web    191690 | actix-web    194978 | may_minihttp    200127 |
|    4 | ohkami       186967 | xitca-web    189218 | ohkami       195495 |
|    5 | xitca-web    185683 | ohkami       188855 | xitca-web    189427 |
|    6 | argan        179058 | argan        179390 | warp         182763 |
|    7 | thruster     177247 | warp         178986 | thruster     181278 |
|    8 | warp         171588 | thruster     178476 | argan        180359 |
|    9 | viz          171407 | axum         172253 | axum         178145 |
|   10 | graphul      170183 | viz          170950 | viz          174972 |
|   11 | axum         166827 | poem         169700 | graphul      169905 |
|   12 | poem         165350 | graphul      169430 | poem         169651 |
|   13 | gotham       163759 | gotham       165822 | gotham       167624 |
|   14 | salvo        159109 | salvo        159664 | salvo        166194 |
|   15 | saphir       157125 | saphir       153501 | saphir       162869 |
|   16 | trillium     146039 | trillium     151474 | trillium     157096 |
|   17 | silent       135295 | silent       132058 | rocket       137857 |
|   18 | iron         127894 | rocket       128939 | silent       136314 |
|   19 | rocket       126818 | astra        126210 | astra        126888 |
|   20 | astra        125790 | iron         118390 | iron         102811 |
|   21 | nickel       115831 | nickel        50048 | nickel        50346 |
|   22 | oxidy         38704 | oxidy         38227 | oxidy         40678 |
|   23 | rouille       17507 | rouille       18595 | rouille       18517 |
|   24 | summer-boot      5650 | tide           5651 | summer-boot      5588 |
|   25 | tide           5646 | summer-boot      5646 | tide           5570 |

## Concurrency: 512

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | may_minihttp    200869 | ntex         204055 | ntex         209000 |
|    2 | ntex         199656 | actix-web    203429 | actix-web    208313 |
|    3 | ohkami       198765 | ohkami       199857 | ohkami       205909 |
|    4 | actix-web    196118 | may_minihttp    196746 | may_minihttp    203451 |
|    5 | thruster     189404 | thruster     190496 | thruster     193530 |
|    6 | xitca-web    188178 | argan        184030 | xitca-web    191372 |
|    7 | argan        184464 | warp         183903 | warp         190759 |
|    8 | warp         179912 | axum         179415 | argan        189823 |
|    9 | viz          178772 | viz          178490 | axum         187744 |
|   10 | axum         176517 | graphul      178387 | viz          184559 |
|   11 | graphul      175860 | xitca-web    177460 | graphul      180391 |
|   12 | poem         174277 | poem         175777 | poem         176863 |
|   13 | gotham       168676 | gotham       173472 | salvo        174104 |
|   14 | salvo        165989 | salvo        165775 | gotham       173206 |
|   15 | saphir       161958 | saphir       161863 | saphir       168340 |
|   16 | trillium     146130 | trillium     147793 | trillium     158212 |
|   17 | silent       131186 | silent       129815 | silent       135799 |
|   18 | rocket       129308 | rocket       127426 | rocket       134891 |
|   19 | astra        123743 | astra        122511 | astra        128434 |
|   20 | iron         116740 | nickel        92890 | nickel       117560 |
|   21 | nickel        50371 | iron          53745 | iron          53064 |
|   22 | oxidy         39115 | oxidy         39177 | oxidy         40942 |
|   23 | rouille       17846 | rouille       17916 | rouille       17385 |
|   24 | summer-boot     11135 | summer-boot     11010 | tide          11026 |
|   25 | tide          10890 | tide          10809 | summer-boot     10782 |
