use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::RangeInclusive;

type Ranges = [RangeInclusive<usize>; 2];

fn main() {
    let input = "departure location: 32-69 or 86-968
departure station: 27-290 or 301-952
departure platform: 47-330 or 347-956
departure track: 46-804 or 826-956
departure date: 25-302 or 320-959
departure time: 29-885 or 893-961
arrival location: 33-643 or 649-963
arrival station: 29-135 or 151-973
arrival platform: 50-648 or 674-961
arrival track: 45-761 or 767-971
class: 46-703 or 725-951
duration: 47-244 or 257-957
price: 49-195 or 209-956
route: 44-368 or 393-968
row: 48-778 or 797-954
seat: 31-421 or 427-964
train: 42-229 or 245-961
type: 31-261 or 281-964
wagon: 36-428 or 445-967
zone: 30-906 or 923-960

your ticket:
157,89,103,59,101,181,109,127,67,173,151,97,107,167,61,131,53,163,179,113

nearby tickets:
463,746,834,524,530,944,558,680,416,986,364,677,850,160,641,99,898,360,860,695
131,550,365,282,759,549,212,841,933,177,109,407,859,5,560,119,929,178,725,769
216,726,353,875,585,448,513,866,55,266,696,173,367,684,99,624,561,735,880,347
775,949,284,404,832,445,4,368,702,492,577,133,930,170,212,642,213,881,616,67
948,608,525,771,739,732,556,615,843,447,254,605,573,838,399,104,285,568,59,112
491,225,446,229,585,462,159,898,365,811,538,469,223,364,943,931,860,835,607,864
414,164,269,604,564,592,608,179,593,397,326,740,774,445,525,215,354,518,446,776
880,224,616,325,182,619,635,875,580,78,499,699,225,158,408,119,400,555,514,591
404,53,154,419,698,67,126,607,519,576,411,51,636,554,856,900,571,133,136,774
758,87,214,54,193,99,400,544,452,63,481,559,488,897,867,463,426,946,284,189
489,447,687,643,851,104,172,528,551,694,685,101,772,172,575,100,838,927,992,881
287,871,570,537,130,290,492,510,163,477,751,273,802,854,624,228,530,591,640,801
406,830,574,569,627,886,547,850,897,773,362,687,398,359,102,179,904,480,121,553
110,879,900,570,508,612,944,408,160,505,767,702,645,65,753,588,463,871,403,359
454,210,123,894,882,285,112,625,936,135,944,88,153,509,852,124,861,997,620,348
185,302,106,452,607,408,111,875,204,861,769,545,499,460,541,895,329,756,329,929
510,24,65,939,885,469,175,587,837,516,478,593,321,696,734,525,107,227,459,694
449,461,195,330,768,501,308,869,747,837,935,109,125,212,212,410,405,358,323,740
324,834,284,411,181,668,548,904,900,105,112,413,687,499,281,323,624,175,214,595
222,110,150,626,488,688,185,593,738,409,619,419,351,87,836,172,474,613,539,641
102,107,614,445,524,414,550,527,556,132,64,767,586,830,226,638,60,900,197,895
173,508,122,768,479,624,554,177,496,726,536,853,607,640,167,845,415,536,248,859
507,617,67,881,585,97,132,769,484,245,799,301,101,405,682,936,578,680,87,756
491,847,160,221,94,832,889,120,576,120,124,756,847,113,169,155,688,217,400,573
744,524,90,867,548,906,895,867,58,682,326,733,354,168,199,753,729,770,447,480
282,89,694,575,738,862,879,131,542,539,590,605,697,675,631,301,834,571,765,325
553,414,732,155,924,108,123,592,321,558,646,624,688,456,51,729,185,193,420,564
259,454,89,579,475,240,290,90,883,740,88,615,856,348,474,101,486,539,98,413
620,347,175,930,604,755,662,733,867,157,458,58,367,609,926,545,219,69,545,593
58,121,126,826,263,218,219,456,691,536,66,193,701,354,564,866,676,182,804,948
159,860,447,828,361,162,523,544,641,629,869,477,894,102,751,799,396,395,650,846
200,65,905,393,856,56,874,906,949,412,741,570,56,479,600,541,578,348,734,365
694,830,665,745,694,590,590,640,111,611,639,770,566,551,258,531,901,585,736,504
626,219,100,934,573,583,847,949,562,847,739,672,559,585,638,527,771,826,947,131
471,608,301,588,109,586,209,537,835,136,559,156,829,87,526,610,494,862,580,117
319,158,288,158,897,100,453,166,194,327,609,756,58,412,213,133,628,518,942,59
452,468,575,778,577,854,604,59,899,943,697,827,715,609,565,464,576,588,542,483
676,827,772,460,875,415,587,218,660,577,612,731,697,122,94,502,101,301,195,210
635,829,732,558,114,804,626,503,408,13,501,882,454,452,108,938,471,834,502,617
283,53,517,497,628,552,884,194,560,938,552,833,531,278,218,896,618,878,837,118
76,493,861,123,110,117,680,50,515,776,842,567,549,777,803,459,517,52,563,257
887,861,214,872,487,582,641,741,636,186,364,563,552,633,630,350,151,902,172,165
161,478,353,777,361,596,501,879,642,420,456,907,634,826,852,840,505,923,895,192
905,510,756,415,772,533,153,799,407,571,171,425,117,229,610,631,187,65,850,774
223,676,134,129,261,211,175,191,411,224,304,544,777,864,53,61,51,900,529,637
272,747,598,799,458,534,747,591,218,545,571,183,157,946,557,622,126,866,396,591
108,261,879,156,842,466,285,738,771,595,52,55,463,518,665,750,880,798,778,703
90,322,517,743,456,911,579,290,923,126,848,868,599,870,628,590,120,544,353,546
861,477,711,754,420,357,257,880,689,559,748,627,508,171,770,469,223,728,884,414
60,538,906,571,902,494,284,493,258,745,124,684,94,646,451,90,850,180,175,514
475,866,326,566,937,975,846,491,456,158,830,403,349,583,557,57,535,883,641,493
498,580,467,116,614,615,629,161,528,454,81,474,843,640,740,96,848,544,99,530
526,628,678,725,881,394,742,773,837,762,414,363,211,550,52,106,60,619,61,539
152,618,802,64,205,828,480,595,898,895,479,527,395,281,95,301,117,357,839,855
749,760,326,866,176,127,184,610,524,703,365,82,559,68,118,828,558,587,520,170
905,457,569,641,610,849,226,223,517,740,407,843,905,738,360,115,92,228,59,987
126,744,503,144,929,684,617,322,220,607,590,584,687,127,941,228,881,415,500,945
475,495,582,51,57,595,400,217,646,134,50,153,185,212,698,876,228,347,736,604
518,770,797,608,499,906,702,946,622,173,820,580,564,68,601,774,728,625,894,932
320,700,588,572,539,470,570,548,556,859,767,284,245,948,411,924,830,935,288,692
66,497,934,756,122,550,948,692,590,867,104,402,551,766,165,637,405,214,184,288
363,675,211,426,549,115,321,690,595,566,867,843,883,220,598,54,604,837,483,420
341,832,534,576,538,760,522,521,176,685,482,472,498,882,697,397,739,947,54,368
777,469,466,123,111,613,236,846,494,191,218,575,800,611,498,193,364,932,633,188
865,258,559,516,587,521,678,100,217,517,744,765,499,176,740,826,325,634,357,560
429,505,736,515,495,702,938,281,152,680,925,193,880,117,494,841,631,110,576,123
631,942,693,804,610,882,716,228,527,490,135,170,498,675,211,363,229,900,415,400
62,900,736,564,483,99,512,946,210,209,767,92,833,187,117,412,898,627,749,711
768,427,536,695,639,730,604,713,470,699,692,555,748,865,216,95,642,630,496,877
505,626,759,772,151,590,227,505,849,680,551,580,203,223,836,579,754,408,288,168
582,556,557,870,573,352,601,455,422,406,133,548,125,874,866,126,65,497,447,902
174,62,357,591,219,836,528,588,903,129,864,335,525,851,124,731,397,754,880,104
286,179,703,646,261,192,475,568,700,803,630,761,349,360,529,769,876,330,797,407
575,63,937,450,943,129,484,399,159,642,942,462,417,368,406,506,694,990,702,570
640,771,349,527,99,353,905,590,897,599,488,89,804,768,526,450,516,827,562,202
852,584,998,584,584,406,726,868,554,690,587,349,451,69,104,733,397,183,773,156
554,445,220,417,758,385,797,519,637,55,734,415,838,697,487,871,614,884,836,495
425,881,508,865,497,774,598,102,134,322,360,350,676,408,211,559,606,416,474,460
288,204,800,943,774,752,302,362,729,349,132,877,499,132,511,414,935,64,120,769
864,638,568,282,504,646,394,114,774,580,325,513,552,258,413,868,905,826,155,469
114,464,747,63,428,822,829,503,846,551,695,413,355,68,929,503,501,767,493,948
583,577,250,55,521,542,777,507,613,630,109,678,635,752,804,700,875,495,938,410
898,217,596,764,944,404,180,555,642,903,861,361,394,613,65,570,170,453,524,564
401,849,744,590,545,161,503,568,159,286,327,213,865,348,529,686,330,769,423,327
841,743,944,323,898,538,505,593,929,578,322,169,553,406,426,616,539,445,120,630
861,470,636,747,468,900,619,897,548,310,939,171,111,755,328,901,360,409,553,183
60,878,648,929,580,54,89,499,135,552,449,735,470,776,906,530,745,767,740,185
221,496,747,508,894,421,535,289,800,457,257,176,517,422,549,114,534,287,686,489
210,396,367,87,588,404,112,157,761,767,353,105,906,9,527,174,634,178,619,605
699,406,577,742,561,589,544,289,864,93,333,732,726,61,583,428,605,261,684,360
353,422,898,168,214,281,738,363,462,674,194,469,554,571,876,566,445,498,159,259
88,684,191,496,512,121,632,538,675,456,745,306,935,773,497,550,778,853,933,756
285,399,191,364,132,634,545,799,428,106,450,802,906,613,520,983,931,526,526,128
393,351,222,184,126,290,874,492,799,189,483,947,24,61,568,355,751,680,848,939
850,550,321,705,621,882,401,362,180,539,804,568,466,356,451,50,863,902,525,400
626,364,59,727,610,356,112,172,518,594,213,157,949,480,501,171,182,133,271,284
683,329,328,173,903,686,830,853,69,893,633,897,327,492,874,365,982,834,581,167
158,631,725,732,94,157,100,924,847,122,547,21,520,855,572,494,928,726,510,604
448,416,521,295,804,260,833,885,897,257,107,285,697,186,94,133,327,926,395,695
499,360,700,18,217,323,50,462,100,66,399,635,327,516,935,399,534,451,680,727
871,561,559,151,745,638,735,165,540,506,676,941,167,200,491,797,485,357,462,159
882,678,465,103,803,885,322,517,630,742,258,189,274,942,93,183,395,771,691,421
777,683,112,67,686,934,410,449,729,699,477,192,749,116,559,66,653,743,601,158
873,874,327,695,835,366,675,729,166,545,613,142,162,219,593,546,427,152,902,636
701,834,770,405,394,868,921,186,326,458,748,696,801,183,677,86,838,447,507,799
739,744,597,226,167,94,92,662,797,163,603,636,368,476,289,355,349,428,632,858
356,464,480,122,100,510,178,549,418,649,614,66,736,61,131,703,930,454,696,501
446,329,850,898,411,483,220,937,421,510,738,514,603,492,706,398,62,494,520,99
682,58,630,211,127,544,119,461,867,471,765,929,219,217,881,160,420,490,483,363
606,835,897,563,150,679,367,419,536,899,537,117,574,512,577,215,612,702,508,941
446,635,456,940,851,216,255,579,543,173,229,614,66,585,179,428,944,634,696,607
643,408,501,351,157,737,607,581,173,53,733,941,748,363,468,860,480,548,713,411
460,984,360,156,166,167,753,220,60,161,834,491,574,935,696,469,258,620,105,737
233,284,803,874,698,842,833,633,526,604,50,697,216,827,466,862,756,864,610,358
815,496,58,564,220,507,581,742,185,401,118,635,931,760,393,857,893,585,484,193
946,322,897,489,602,448,211,773,148,937,774,178,924,947,479,539,486,870,872,576
628,506,681,59,172,597,146,758,301,758,703,934,547,570,695,570,523,748,176,573
725,54,930,421,631,177,19,616,357,98,476,61,229,356,155,626,60,873,454,110
415,419,229,272,700,531,568,50,726,459,133,745,869,580,108,732,215,470,868,924
219,175,177,356,847,879,211,932,853,675,732,753,425,215,579,695,492,257,415,929
461,583,156,455,127,506,471,759,347,940,368,726,223,185,450,327,112,901,978,505
679,62,609,477,744,123,263,583,568,476,743,162,761,760,679,518,526,939,602,285
194,367,619,336,62,851,195,900,619,407,773,640,543,322,563,622,600,696,679,559
897,591,404,79,213,577,803,605,556,939,949,594,52,893,286,476,61,496,797,321
906,609,742,572,602,933,340,544,940,101,171,847,609,218,134,285,213,515,881,692
539,445,773,870,756,82,754,855,687,396,587,167,281,116,175,427,803,54,797,537
295,638,690,156,700,124,896,192,685,538,699,469,411,92,103,327,101,181,852,598
688,640,58,258,229,175,603,566,883,745,120,751,361,983,895,152,871,776,565,475
571,110,473,844,182,325,282,474,800,848,720,872,580,463,642,637,302,937,565,54
884,477,454,11,851,768,155,427,617,894,395,471,127,756,119,349,499,933,569,367
755,159,620,255,403,541,639,215,619,155,121,880,512,758,132,169,859,488,831,511
733,869,737,830,169,174,163,194,931,891,99,209,743,101,570,934,100,156,642,63
474,487,517,162,426,690,355,924,188,467,195,189,67,623,537,160,51,257,750,163
773,597,167,448,992,485,524,749,839,414,581,419,69,544,155,302,108,489,210,640
152,488,736,115,55,206,732,756,631,878,261,214,354,465,365,559,576,552,105,155
114,411,759,2,478,283,569,394,607,488,524,678,90,870,856,581,941,844,184,111
152,161,638,357,757,758,328,618,186,96,630,891,428,536,353,68,603,54,702,599
168,170,603,134,769,278,853,447,219,743,526,450,729,457,289,758,842,102,493,758
66,86,681,483,684,730,855,13,731,505,531,558,351,364,550,195,519,365,393,518
857,544,529,602,566,333,499,414,181,778,357,112,134,539,901,135,827,697,854,53
686,461,208,530,879,871,863,727,489,613,893,397,860,732,92,899,459,850,133,420
853,701,562,352,54,574,423,428,495,746,759,355,507,737,540,777,456,179,587,91
195,936,925,221,178,739,161,854,320,257,569,754,141,475,633,418,177,402,106,577
535,329,629,684,877,727,509,750,727,158,537,527,514,461,321,574,769,320,647,679
215,568,691,508,841,295,57,736,172,564,213,597,500,933,479,557,445,288,551,777
478,597,540,116,494,449,10,101,546,611,67,777,482,408,409,130,578,212,636,934
588,857,728,926,620,355,600,358,153,481,753,228,227,702,687,182,710,592,640,604
381,565,728,741,221,522,215,615,590,494,154,569,760,747,926,530,56,411,875,367
931,404,258,462,642,529,347,510,613,473,942,897,617,829,623,420,633,507,989,393
211,449,284,518,593,840,94,133,126,518,144,540,504,521,857,884,602,587,460,941
185,619,688,495,594,748,617,398,626,862,491,207,850,60,548,623,641,777,393,411
898,404,559,524,461,488,898,773,870,59,641,778,528,399,462,54,67,450,82,498
64,588,21,287,465,621,542,452,584,568,745,841,639,395,878,86,120,635,760,481
734,282,753,261,469,180,114,578,284,945,755,715,924,92,799,122,213,113,104,514
107,427,257,703,673,846,258,700,747,216,593,686,736,729,607,850,756,874,804,590
540,549,180,156,496,124,124,871,118,193,596,865,350,110,66,119,1,855,640,464
55,740,893,196,53,63,68,548,896,479,587,525,600,867,119,753,479,948,517,632
846,480,885,573,494,680,358,949,56,498,583,471,193,663,859,452,412,506,566,359
668,934,692,367,732,923,748,99,947,697,222,56,864,492,880,527,495,875,906,451
562,91,529,354,728,423,288,400,467,177,881,221,539,834,942,448,455,178,800,466
480,538,525,463,585,612,474,159,898,463,771,727,674,945,356,86,754,12,602,744
183,935,224,324,287,700,801,755,364,705,68,454,419,861,857,846,356,326,301,533
646,634,626,414,185,173,700,873,754,474,949,52,573,220,851,535,862,601,829,931
866,584,746,402,350,751,754,588,762,863,584,555,580,565,225,868,868,193,88,531
184,847,898,758,509,838,285,100,574,4,92,727,69,89,323,534,570,574,364,700
773,67,591,402,512,753,509,624,213,123,157,298,511,164,501,445,222,401,496,545
447,928,683,937,363,132,755,775,550,595,280,676,466,728,883,676,404,170,624,941
540,455,903,484,302,804,302,742,944,569,186,927,777,400,688,504,546,564,554,15
868,469,102,594,534,622,90,153,228,894,676,501,521,181,609,924,980,185,929,160
214,614,107,586,52,582,112,482,114,757,410,609,827,948,161,893,198,398,212,421
185,209,678,923,576,182,167,134,802,407,352,575,651,120,193,501,421,527,692,101
445,539,361,605,125,522,72,552,623,482,760,326,188,898,750,933,604,692,905,176
511,928,321,747,586,611,626,679,470,117,347,60,765,828,478,858,115,100,170,357
762,751,677,349,694,865,868,619,157,928,165,558,492,829,453,574,727,743,537,496
906,739,628,466,182,715,899,855,361,328,872,412,691,105,865,177,91,500,402,860
719,95,933,551,735,597,690,608,479,847,92,118,827,105,193,728,726,135,482,876
517,468,212,668,895,529,627,128,462,355,357,360,323,301,99,680,693,502,111,394
735,802,625,259,364,760,95,283,928,176,929,524,299,593,840,832,323,129,61,878
948,712,257,854,619,213,540,856,562,883,580,504,739,97,742,445,798,519,604,739
527,695,855,743,459,540,544,536,932,932,872,486,455,248,575,220,473,893,450,158
281,215,570,91,591,489,643,261,274,687,63,65,177,177,846,637,882,404,180,894
106,416,356,926,944,458,19,580,528,215,850,935,927,399,69,761,941,851,730,258
947,257,618,153,184,357,859,320,554,611,297,91,355,487,837,871,412,864,472,122
591,99,86,521,399,396,647,397,848,773,525,93,800,927,454,517,592,893,756,592
103,854,498,88,290,529,130,694,167,282,992,51,874,515,492,538,490,873,678,454
259,945,171,512,639,925,684,830,64,637,642,458,360,349,147,873,330,164,729,364
728,829,483,259,560,685,177,451,177,477,607,680,161,301,101,134,358,20,861,926
733,602,875,641,932,349,947,393,212,678,217,403,636,69,450,541,882,600,297,631
356,64,422,745,638,216,885,90,185,884,154,574,285,607,476,99,550,120,152,839
832,631,942,741,458,353,467,219,605,769,570,516,848,771,465,99,149,348,880,696
169,599,902,490,777,420,349,801,161,197,859,838,932,395,448,933,120,220,731,185
623,558,994,412,58,286,944,155,102,156,850,124,840,896,129,842,465,515,835,127
526,211,403,190,187,408,505,570,559,493,542,356,935,610,255,530,937,258,497,407
680,368,864,745,761,393,179,834,186,754,547,646,827,750,777,57,935,161,159,94
103,452,996,874,450,906,59,738,686,366,168,494,123,185,804,754,775,903,746,97
641,635,768,62,51,762,526,726,931,485,166,840,181,754,475,104,598,870,302,636
895,451,846,129,53,880,125,578,363,469,835,508,604,458,797,942,509,125,20,603
977,128,557,622,728,859,352,465,526,693,213,468,896,829,92,582,96,483,852,868
929,728,845,676,939,461,866,743,480,116,62,734,57,152,894,495,872,707,90,616
676,878,768,543,216,195,868,322,160,603,325,824,415,638,171,488,904,104,847,468
176,843,560,897,112,640,583,556,844,61,203,928,575,528,738,217,258,496,928,733
330,534,456,554,421,227,765,224,742,400,170,457,846,489,610,545,226,122,900,60
505,131,488,894,428,448,571,642,181,411,130,454,583,724,760,452,472,760,281,693
630,446,420,809,826,859,129,187,602,842,287,214,131,694,416,879,770,837,676,769
774,532,176,775,588,677,405,597,715,804,212,58,420,826,281,799,497,284,758,114
215,993,733,325,776,365,325,54,544,285,755,115,774,895,614,885,642,514,738,927
623,602,746,854,760,126,88,465,616,761,758,797,825,834,799,948,458,65,551,799
852,348,522,108,631,124,693,867,544,686,496,703,183,409,23,642,754,284,221,478
247,688,93,479,862,504,290,467,325,535,215,857,925,362,160,112,286,323,96,542
940,353,688,568,903,156,693,877,877,121,624,178,91,140,514,94,532,761,538,744
157,116,217,904,591,552,555,133,729,566,759,248,358,841,522,925,484,163,544,740
417,842,119,612,135,595,815,801,939,906,571,446,557,800,800,617,512,124,586,64
701,604,904,365,252,745,54,566,700,551,689,753,943,477,510,349,492,216,505,118
849,61,899,97,723,642,797,546,105,92,289,832,596,578,108,125,212,281,948,358
214,482,749,168,614,640,208,594,533,574,620,353,590,852,830,57,449,412,628,322
354,776,725,902,412,177,448,682,610,422,462,799,106,213,840,860,410,767,475,321
926,550,130,870,466,255,190,491,322,589,260,837,758,844,478,115,59,638,831,490
215,367,184,170,223,882,511,615,807,748,324,832,416,103,742,801,475,553,851,96
884,394,856,591,564,258,135,835,254,187,497,857,547,681,59,486,506,681,494,827
138,573,394,531,356,105,839,731,593,680,614,523,404,594,65,455,330,221,489,619
881,687,461,352,469,159,174,423,192,539,508,350,535,415,112,905,862,169,679,850
416,488,681,546,606,139,760,427,868,541,616,599,900,504,561,528,505,702,742,948
588,260,559,611,158,153,182,497,901,648,840,94,803,406,627,903,519,513,643,212
870,934,586,799,161,638,852,173,608,322,422,289,925,726,499,468,445,111,897,883
505,881,593,155,448,101,524,206,513,176,736,926,591,68,466,490,895,163,287,259
606,540,415,448,602,400,399,557,859,599,660,325,803,837,288,876,160,690,932,498
528,693,869,93,998,940,865,571,161,185,452,800,701,415,480,521,169,456,802,867
396,534,841,880,727,560,445,737,745,292,326,471,578,288,519,943,871,585,64,949
866,703,281,358,944,994,457,750,627,587,507,680,473,544,933,535,850,211,852,606
547,580,409,510,588,307,517,219,520,757,944,689,126,768,481,189,55,827,874,366
615,560,417,185,924,170,744,415,932,344,88,355,847,490,221,831,395,192,498,773
934,943,836,676,489,173,293,736,471,641,288,500,680,194,596,186,555,228,109,420
914,602,902,874,218,129,492,689,738,118,121,751,942,874,893,855,778,545,857,544
457,325,531,881,682,526,837,223,751,642,209,488,937,410,587,944,547,746,821,352
777,836,861,64,328,506,68,351,365,827,164,351,169,181,930,738,677,690,988,834";

    let mut section = 1;
    let mut ranges_by_field: HashMap<&str, Ranges> = HashMap::new();
    let mut our_ticket: Vec<usize> = vec![];
    let mut nearby_tickets: Vec<Vec<usize>> = vec![];
    for line in input.lines() {
        if line.trim() == "your ticket:" {
            section = 2;
            continue;
        }
        if line.trim() == "nearby tickets:" {
            section = 3;
            continue;
        }
        if line.trim() == "" {
            continue;
        }

        match section {
            1 => {
                let (field_name, ranges) = line.trim().split(": ").collect_tuple().unwrap();
                let (range1, range2) = ranges
                    .split(" or ")
                    .map(|range_str| {
                        let (lo, hi) = range_str.split("-").collect_tuple().unwrap();
                        RangeInclusive::new(lo.parse().unwrap(), hi.parse().unwrap())
                    })
                    .collect_tuple()
                    .unwrap();
                ranges_by_field.insert(field_name, [range1, range2]);
            }
            2 => our_ticket = line.split(",").map(|s| s.parse().unwrap()).collect(),
            3 => nearby_tickets.push(line.split(",").map(|s| s.parse().unwrap()).collect()),
            _ => unimplemented!(),
        }
    }

    let all_ranges = || ranges_by_field.values().flatten();
    let valid_tickets: Vec<Vec<usize>> = nearby_tickets
        .into_iter()
        .filter(|all_values_in_ticket| {
            all_values_in_ticket
                .iter()
                .all(|value| all_ranges().any(|range| range.contains(value)))
        })
        .collect();

    let ticket_length = our_ticket.len();
    let valid_tickets = &valid_tickets;

    let hm = ranges_by_field
        .iter()
        .map(move |(field_name, [range_a, range_b])| {
            (
                field_name,
                (0..ticket_length)
                    .map(move |field_index| {
                        valid_tickets
                            .iter()
                            .enumerate()
                            .filter_map(move |(ticket_id, ticket)| {
                                let field_value = ticket[field_index];
                                if range_a.contains(&field_value) || range_b.contains(&field_value)
                                {
                                    Some(ticket_id)
                                } else {
                                    None
                                }
                            })
                            .collect_vec()
                    })
                    .collect_vec(),
            )
        })
        .collect::<HashMap<_, _>>();

    let mut hm = hm
        .iter()
        .map(|(field_name, fields)| {
            (
                field_name,
                fields
                    .iter()
                    .enumerate()
                    .filter_map(|(field_id, matching_ticket_ids)| {
                        if matching_ticket_ids.len() == valid_tickets.len() {
                            Some(field_id)
                        } else {
                            None
                        }
                    })
                    .collect::<HashSet<_>>(),
            )
        })
        .sorted_by_key(|t| t.1.len())
        .collect::<VecDeque<_>>();

    let mut ticket_index_by_field_name = HashMap::new();
    let mut used_fields = HashSet::new();
    while !hm.is_empty() {
        let (field_name, possible_fields) = hm.pop_front().unwrap();
        let (field_index,) = possible_fields
            .difference(&used_fields)
            .collect_tuple()
            .unwrap();
        ticket_index_by_field_name.insert(field_name, *field_index);
        used_fields = possible_fields;
    }

    dbg!(ticket_index_by_field_name
        .iter()
        .filter(|(field, _)| field.starts_with("departure "))
        .map(|(_, field_index)| our_ticket[*field_index])
        .product::<usize>());
}
