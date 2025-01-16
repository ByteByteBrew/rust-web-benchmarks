# Web Framework Benchmark Results

## Test Configuration

- Date: 2025-01-16 17:16:57
## Hardware Information
- CPU: Intel(R) Xeon(R) Gold 6267C CPU @ 2.60GHz (8 cores)
- Memory: 32095 MB

# Performance Rankings (Requests/sec)

## Concurrency: 64

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | ntex         169826 | ntex         168288 | actix-web    173313 |
|    2 | actix-web    168174 | actix-web    168092 | ntex         172753 |
|    3 | may_minihttp    166794 | may_minihttp    164038 | may_minihttp    166261 |
|    4 | xitca-web    158655 | xitca-web    157736 | xitca-web    162520 |
|    5 | ohkami       137861 | thruster     139890 | ohkami       151065 |
|    6 | argan        134212 | ohkami       139620 | warp         139953 |
|    7 | viz          133436 | argan        138872 | thruster     139512 |
|    8 | thruster     132858 | warp         138050 | argan        138710 |
|    9 | graphul      131455 | viz          133965 | graphul      137304 |
|   10 | poem         129282 | salvo        131386 | axum         136339 |
|   11 | salvo        128843 | axum         130854 | salvo        133452 |
|   12 | axum         128305 | graphul      130238 | iron         131567 |
|   13 | iron         127204 | iron         130035 | viz          130413 |
|   14 | gotham       122227 | poem         128731 | gotham       128202 |
|   15 | warp         122221 | astra        122172 | poem         125987 |
|   16 | saphir       122207 | gotham       119339 | trillium     121432 |
|   17 | trillium     117797 | saphir       117737 | silent       118823 |
|   18 | astra        117393 | silent       116411 | saphir       115791 |
|   19 | silent       114900 | trillium     113905 | astra        115303 |
|   20 | rocket       107416 | rocket       107245 | rocket       110744 |
|   21 | nickel        79927 | nickel        80170 | nickel        80037 |
|   22 | oxidy         34170 | oxidy         34601 | oxidy         33503 |
|   23 | rouille       18077 | rouille       17945 | rouille       17987 |
|   24 | tide           1419 | summer-boot      1416 | summer-boot      1418 |
|   25 | summer-boot      1418 | tide           1412 | tide           1416 |

## Concurrency: 256

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | ntex         189191 | actix-web    187732 | ntex         198472 |
|    2 | actix-web    187545 | ntex         186708 | actix-web    193341 |
|    3 | may_minihttp    184169 | may_minihttp    185199 | ohkami       187625 |
|    4 | ohkami       177531 | ohkami       175937 | may_minihttp    184617 |
|    5 | xitca-web    174932 | xitca-web    173726 | xitca-web    176283 |
|    6 | thruster     165787 | thruster     171615 | thruster     171487 |
|    7 | argan        165273 | argan        171376 | warp         170323 |
|    8 | graphul      158815 | viz          163568 | argan        167241 |
|    9 | warp         158672 | warp         157991 | graphul      165830 |
|   10 | viz          157993 | graphul      155306 | axum         165284 |
|   11 | poem         156535 | axum         154896 | viz          163632 |
|   12 | axum         153937 | salvo        154137 | salvo        158494 |
|   13 | salvo        150821 | poem         153739 | poem         155287 |
|   14 | gotham       148377 | gotham       143717 | saphir       150173 |
|   15 | saphir       146908 | saphir       138177 | gotham       149744 |
|   16 | trillium     135210 | trillium     137268 | trillium     145152 |
|   17 | silent       122091 | iron         131761 | silent       130824 |
|   18 | rocket       121667 | silent       126940 | iron         128454 |
|   19 | astra        118724 | rocket       122052 | rocket       124018 |
|   20 | iron         116935 | astra        109923 | astra        108175 |
|   21 | nickel        48210 | nickel        48102 | nickel        48580 |
|   22 | oxidy         38133 | oxidy         37029 | oxidy         38900 |
|   23 | rouille       16628 | rouille       16552 | rouille       16994 |
|   24 | tide           5645 | summer-boot      5645 | summer-boot      5650 |
|   25 | summer-boot      5603 | tide           5635 | tide           5616 |

## Concurrency: 512

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | ntex         195070 | ntex         193516 | ntex         198842 |
|    2 | ohkami       190155 | may_minihttp    191108 | ohkami       197468 |
|    3 | actix-web    189646 | ohkami       190312 | actix-web    196970 |
|    4 | may_minihttp    188447 | actix-web    188540 | may_minihttp    189317 |
|    5 | argan        179246 | thruster     179896 | thruster     186154 |
|    6 | thruster     174515 | argan        178625 | xitca-web    184195 |
|    7 | xitca-web    173147 | xitca-web    177543 | argan        181515 |
|    8 | warp         169709 | viz          170676 | warp         176026 |
|    9 | viz          168989 | warp         169028 | viz          174316 |
|   10 | poem         165337 | graphul      165943 | axum         173092 |
|   11 | axum         165165 | poem         164964 | graphul      170172 |
|   12 | graphul      164021 | axum         160995 | poem         165631 |
|   13 | salvo        156010 | salvo        157755 | salvo        162954 |
|   14 | gotham       154148 | gotham       152738 | gotham       159024 |
|   15 | saphir       153254 | saphir       141592 | saphir       158149 |
|   16 | trillium     137132 | trillium     137455 | trillium     140066 |
|   17 | silent       124189 | silent       124557 | silent       127749 |
|   18 | rocket       120439 | rocket       117606 | rocket       123987 |
|   19 | astra        117274 | astra        117040 | astra        102531 |
|   20 | iron         102424 | iron          52161 | iron          52124 |
|   21 | nickel        48651 | nickel        48768 | nickel        49221 |
|   22 | oxidy         37955 | oxidy         36565 | oxidy         39210 |
|   23 | rouille       16517 | rouille       16417 | rouille       15884 |
|   24 | summer-boot     10801 | tide          11008 | summer-boot     10945 |
|   25 | tide          10699 | summer-boot     10915 | tide          10876 |
