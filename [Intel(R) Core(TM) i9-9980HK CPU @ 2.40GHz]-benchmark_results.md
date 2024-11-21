# Web Framework Benchmark Results

## Test Configuration

- Date: 2024-11-21 13:02:37
## Hardware Information
- CPU: Intel(R) Core(TM) i9-9980HK CPU @ 2.40GHz (16 cores)
- Memory: 7997 MB

# Performance Rankings (Requests/sec)

## Concurrency: 64

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | actix-web    386308 | actix-web    389177 | xitca-web    430523 |
|    2 | xitca-web    371829 | may_minihttp    360497 | actix-web    415344 |
|    3 | may_minihttp    356202 | xitca-web    339127 | may_minihttp    373073 |
|    4 | ntex         331632 | ntex         337652 | ntex         356612 |
|    5 | ohkami       263275 | ohkami       264479 | ohkami       273364 |
|    6 | iron         249697 | iron         255645 | iron         259140 |
|    7 | argan        230100 | warp         231318 | warp         241770 |
|    8 | warp         222189 | argan        221019 | argan        226009 |
|    9 | axum         203725 | poem         205267 | axum         224035 |
|   10 | poem         202832 | nickel       200855 | poem         210864 |
|   11 | nickel       201062 | axum         196684 | viz          203500 |
|   12 | salvo        184059 | viz          193885 | silent       199665 |
|   13 | silent       183337 | silent       187693 | salvo        190766 |
|   14 | gotham       178875 | salvo        183323 | nickel       185089 |
|   15 | graphul      178849 | gotham       173668 | graphul      178623 |
|   16 | viz          173884 | graphul      170625 | gotham       175434 |
|   17 | rocket       150569 | rocket       149001 | rocket       154968 |
|   18 | oxidy         77185 | oxidy         67876 | oxidy         85575 |
|   19 | tide           1558 | tide           1555 | summer-boot      1555 |
|   20 | summer-boot      1557 | summer-boot      1554 | tide           1554 |

## Concurrency: 256

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | actix-web    584739 | ntex         617731 | actix-web    666441 |
|    2 | ntex         577242 | actix-web    597987 | ntex         663393 |
|    3 | xitca-web    556765 | xitca-web    587153 | xitca-web    605419 |
|    4 | ohkami       520973 | ohkami       542167 | ohkami       571928 |
|    5 | argan        448698 | argan        451932 | argan        478219 |
|    6 | may_minihttp    420630 | may_minihttp    442929 | may_minihttp    461347 |
|    7 | warp         410358 | warp         417726 | warp         434514 |
|    8 | axum         364625 | viz          367782 | axum         392290 |
|    9 | viz          352028 | axum         361014 | viz          389375 |
|   10 | graphul      344351 | graphul      348724 | graphul      374463 |
|   11 | gotham       332179 | poem         339225 | poem         358718 |
|   12 | poem         326652 | gotham       338961 | gotham       355313 |
|   13 | salvo        325291 | salvo        325934 | salvo        346498 |
|   14 | iron         242425 | silent       242774 | iron         255677 |
|   15 | silent       236871 | iron         238294 | silent       245983 |
|   16 | rocket       226509 | rocket       218692 | rocket       233889 |
|   17 | nickel       198152 | nickel       131516 | nickel       213735 |
|   18 | oxidy         78961 | oxidy         68682 | oxidy         83696 |
|   19 | tide           6024 | tide           6058 | summer-boot      6081 |
|   20 | summer-boot      6020 | summer-boot      6046 | tide           6068 |

## Concurrency: 512

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | ohkami       637476 | ohkami       693136 | ohkami       746397 |
|    2 | actix-web    598112 | actix-web    611405 | actix-web    675273 |
|    3 | xitca-web    576764 | ntex         601403 | ntex         647373 |
|    4 | ntex         573725 | xitca-web    578459 | xitca-web    625888 |
|    5 | argan        531532 | argan        549642 | argan        584858 |
|    6 | warp         496088 | warp         517626 | warp         571693 |
|    7 | may_minihttp    463126 | may_minihttp    483769 | may_minihttp    511767 |
|    8 | axum         429573 | axum         438394 | axum         498286 |
|    9 | viz          420404 | gotham       404540 | viz          448679 |
|   10 | graphul      402476 | viz          400476 | graphul      441765 |
|   11 | gotham       392538 | poem         400372 | poem         424253 |
|   12 | poem         386946 | graphul      396631 | gotham       418507 |
|   13 | salvo        384563 | salvo        395892 | salvo        402148 |
|   14 | iron         239393 | iron         248471 | rocket       252437 |
|   15 | silent       235744 | rocket       234237 | silent       244563 |
|   16 | rocket       230400 | silent       230874 | iron         224801 |
|   17 | nickel       159108 | nickel       144601 | nickel       131272 |
|   18 | oxidy         74667 | oxidy         66444 | oxidy         83274 |
|   19 | summer-boot     11860 | tide          11817 | tide          12015 |
|   20 | tide          11819 | summer-boot     11744 | summer-boot     11824 |
