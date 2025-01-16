# Web Framework Benchmark Results

## Test Configuration

- Date: 2025-01-16 17:14:25
## Hardware Information
- CPU: Intel(R) Core(TM) i9-9980HK CPU @ 2.40GHz (16 cores)
- Memory: 16049 MB

# Performance Rankings (Requests/sec)

## Concurrency: 64

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | actix-web    400583 | actix-web    413089 | actix-web    445566 |
|    2 | xitca-web    395766 | xitca-web    406345 | xitca-web    422299 |
|    3 | may_minihttp    391177 | may_minihttp    390453 | may_minihttp    416460 |
|    4 | ntex         367327 | ntex         378713 | ntex         393893 |
|    5 | ohkami       293216 | ohkami       301247 | ohkami       314860 |
|    6 | iron         264157 | iron         283771 | iron         287517 |
|    7 | thruster     244107 | thruster     255719 | thruster     258571 |
|    8 | argan        233899 | astra        243777 | astra        250987 |
|    9 | astra        230531 | warp         235905 | warp         237366 |
|   10 | viz          206208 | argan        233230 | argan        237358 |
|   11 | saphir       203809 | viz          208835 | axum         227644 |
|   12 | graphul      200789 | axum         205504 | viz          225482 |
|   13 | axum         197856 | nickel       204508 | saphir       218687 |
|   14 | gotham       196295 | saphir       202350 | graphul      210830 |
|   15 | nickel       196073 | poem         200094 | poem         208610 |
|   16 | warp         194302 | gotham       199648 | gotham       194332 |
|   17 | poem         192838 | graphul      196737 | salvo        192572 |
|   18 | silent       186544 | salvo        189961 | trillium     190196 |
|   19 | salvo        183935 | silent       183447 | nickel       189923 |
|   20 | trillium     173856 | trillium     172918 | silent       185487 |
|   21 | rocket       157175 | rocket       152091 | rocket       163429 |
|   22 | oxidy         79677 | oxidy         75084 | oxidy         94173 |
|   23 | rouille       26275 | rouille       27003 | rouille       26357 |
|   24 | summer-boot      1548 | summer-boot      1547 | summer-boot      1551 |
|   25 | tide           1547 | tide           1546 | tide           1548 |

## Concurrency: 256

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | actix-web    638000 | actix-web    663793 | xitca-web    721173 |
|    2 | xitca-web    630888 | ntex         663585 | ntex         710774 |
|    3 | ntex         629601 | xitca-web    660112 | actix-web    702786 |
|    4 | ohkami       519502 | ohkami       542960 | ohkami       577404 |
|    5 | may_minihttp    485179 | may_minihttp    492859 | may_minihttp    514509 |
|    6 | thruster     442212 | thruster     460172 | thruster     479922 |
|    7 | argan        441785 | argan        452898 | argan        467910 |
|    8 | warp         411652 | warp         430255 | warp         451274 |
|    9 | viz          389052 | viz          385319 | axum         447428 |
|   10 | gotham       365451 | graphul      370285 | graphul      407415 |
|   11 | graphul      364247 | axum         369815 | saphir       406321 |
|   12 | axum         360029 | gotham       364608 | viz          402634 |
|   13 | saphir       344873 | poem         347175 | gotham       384076 |
|   14 | poem         343645 | saphir       340657 | poem         370469 |
|   15 | salvo        338115 | salvo        337625 | salvo        342735 |
|   16 | trillium     272306 | trillium     278967 | trillium     292945 |
|   17 | iron         262859 | iron         277576 | iron         270403 |
|   18 | silent       233966 | silent       232091 | rocket       237786 |
|   19 | rocket       227444 | rocket       227715 | astra        237598 |
|   20 | astra        207613 | astra        203370 | silent       233980 |
|   21 | nickel       200214 | nickel       194558 | nickel       201944 |
|   22 | oxidy         83477 | oxidy         76391 | oxidy         96510 |
|   23 | rouille       26194 | rouille       25712 | rouille       25915 |
|   24 | tide           6189 | tide           6042 | summer-boot      5992 |
|   25 | summer-boot      6005 | summer-boot      5982 | tide           5986 |

## Concurrency: 512

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | ohkami       662929 | ohkami       730024 | ohkami       792566 |
|    2 | actix-web    655095 | ntex         680026 | actix-web    740060 |
|    3 | ntex         651761 | xitca-web    666830 | ntex         734329 |
|    4 | xitca-web    620504 | actix-web    659122 | xitca-web    703748 |
|    5 | thruster     592917 | thruster     636773 | thruster     678201 |
|    6 | argan        547619 | argan        567570 | argan        618936 |
|    7 | may_minihttp    533035 | may_minihttp    546451 | warp         603062 |
|    8 | warp         519178 | warp         543357 | may_minihttp    573804 |
|    9 | viz          472016 | axum         469461 | axum         570155 |
|   10 | graphul      466805 | viz          462193 | viz          521116 |
|   11 | axum         439190 | graphul      446440 | saphir       498586 |
|   12 | saphir       432564 | gotham       441161 | graphul      485985 |
|   13 | poem         422937 | poem         437334 | gotham       475563 |
|   14 | gotham       420857 | salvo        424450 | poem         445448 |
|   15 | salvo        412566 | saphir       419408 | salvo        442380 |
|   16 | trillium     315911 | trillium     311628 | trillium     337908 |
|   17 | iron         253958 | iron         274665 | rocket       269676 |
|   18 | rocket       250927 | rocket       260236 | silent       247241 |
|   19 | silent       237282 | silent       237876 | iron         244677 |
|   20 | astra        210197 | astra        210338 | astra        232562 |
|   21 | nickel       155603 | nickel       169760 | nickel       159159 |
|   22 | oxidy         81711 | oxidy         74771 | oxidy         95549 |
|   23 | rouille       25886 | rouille       26510 | rouille       25963 |
|   24 | tide          12072 | tide          11972 | tide          11746 |
|   25 | summer-boot     11723 | summer-boot     11839 | summer-boot     11732 |
