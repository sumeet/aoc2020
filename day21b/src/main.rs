use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let mut possible_ingredients_by_allergen: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut all_ingredient_occurrences: Vec<&str> = vec![];

    for line in INPUT.lines() {
        let line = line.trim_end_matches(')');
        let (ingredients, allergens) = line.split(" (contains ").collect_tuple().unwrap();
        let ingredients = ingredients.split(" ").collect::<HashSet<_>>();
        all_ingredient_occurrences.extend(&ingredients);
        let allergens = allergens.split(", ");

        for allergen in allergens {
            let possible_ingredients = possible_ingredients_by_allergen
                .entry(allergen)
                .or_insert_with(|| ingredients.clone());
            *possible_ingredients = possible_ingredients
                .intersection(&ingredients)
                .cloned()
                .collect();
        }
    }

    let mut known_allergens = HashMap::new();
    while !possible_ingredients_by_allergen.is_empty() {
        let (allergen, ingredient) = possible_ingredients_by_allergen
            .iter()
            .find(|(_, ingredients)| ingredients.len() == 1)
            .map(|(allergen, ingredients)| {
                (
                    allergen.to_string(),
                    ingredients.iter().next().unwrap().to_string(),
                )
            })
            .unwrap();
        known_allergens.insert(allergen.clone(), ingredient.clone());
        possible_ingredients_by_allergen.remove(allergen.as_str());
        for possible_ingredients in possible_ingredients_by_allergen.values_mut() {
            possible_ingredients.remove(ingredient.as_str());
        }
    }
    dbg!(known_allergens
        .iter()
        .sorted()
        .map(|(_allergen, ingredient)| ingredient)
        .join(","));
    // dbg!(possible_ingredients_by_allergen);
    // let might_be_allergens = possible_ingredients_by_allergen
    //     .values()
    //     .flatten()
    //     .cloned()
    //     .collect::<HashSet<_>>();
    //
    // dbg!(all_ingredient_occurrences
    //     .iter()
    //     .filter(|ingredient| !might_be_allergens.contains(*ingredient))
    //     .count());
}

const INPUT: &str = "bmhn hcpvz klpx fshdvz kxrz spg fmgxh gnspg pgqxvc nsg vrqbct nvzj pqnlq tphvg rzzs crkvkk fxzh xgtms rjgfr ldkp gggnm kskcfv fkrkk szc mfdv psxvf jpsnjbt cskbmx ppbqpx lqlt bmgtf tstp tlhqvhq tktzx vcck bxdbkvtt tzxcmr mxbs lxxm fdzg jdnrq mmkmt hrv qstvls hxbp mmpphm pdhv mmdh knvg fblzjv qqgjft nqfsk gqzzzp fpk pqjk xgcjxkkm jptrc cjdmk kspj xbt hsxpqftn cjkrc chs qnhmjbz rzhb gsfqhcd jzrjtn jrmr nptr mvkxqh ttmphk nnhl xjgzz (contains fish, shellfish)
fpk hcrbxt vbztx jrmr vrqbct szc hjm jdnrq xjgzz gxtlpx sfdc pqnlq pbt tzxcmr zdmxsgp jbp xjdchp jpsnjbt mdjr tphvg lcvcng kskcfv tlhqvhq jzrjtn cjdmk tktzx vqdpfb fblzjv gqzzzp ctsl gksjpm mpbm pcrmsb qqgjft xrmf cskbmx rzhb mvkxqh hrv ckz mxbs smppjj rjgfr bmhn hcpvz vndglc gcczzsd fxzh tstp xlxknk sdzzhnfc gmblk gnc psxvf hdfqc pzvb ttmphk sbh hlsvmjh xrxrs (contains wheat, nuts)
dkfvl nnhl mpbm djbb pzfmh pqdtkpd mvkxqh cskbmx jbp sfqjtk klhrx fmgxh gqb nptr vbztx fdxgf fkrkk fvgb mhdf fshdvz hqpfl kvdbxp kspj ffpq chs vqdpfb hxpxh tktzx gqzzzp tzxcmr cjdmk jxkn pcrmsb vxchx qnhmjbz hlsvmjh ttmphk tqxcv nsg fxzh fblzjv lqlt bmgtf jrmr bmhn rzzs hcpvz zmzz sbcntv jssvhx tlhqvhq jcjt (contains fish)
gmblk cfjkt hjm nqfsk szc tzxcmr fblzjv lqlt nvql jpsnjbt fxzh gqb vnzdgh gcczzsd qdjfg kspj mhdf nthkh vqdpfb xppzqqz ldkp ppbqpx bmhn ghfxfs qstvls hxpxh pcrmsb bkqj nsg hjlv lxxm fkrkk rzzs vnmbz ffcxn klhrx vbztx zxpnt sfqjtk gnc hrv nnhl chs xlxknk gxtlpx ffrxl cjdmk zdmxsgp fxvcg gxlp kbgcsp fvgb jcjt zbmcz ttmphk nqx jrmr cskbmx pstjn tphvg hpjnl grxfxfr rgbj zgg xslk fshdvz pqnlq hlktbd pqpvvm pqjk hxbp hddxgp rttkb xbt (contains dairy, shellfish, fish)
fpk klpx pgqxvc hcpvz nptr jxkn spg hvpx zzgjdt nvzj gqb pzfmh vxchx qdjfg bmgtf zqb tftlfk jptrc ffrxl kxrz hlsvmjh fxvcg tzxcmr cskbmx vrqbct pdhv zbmcz kvdbxp bmhn xlxknk jbtf grxfxfr hsxpqftn cfjkt mngvb jpsnjbt jrmr hjnnd sbcntv kskcfv klhrx rtqxnnm pstjn cjkrc cjdmk gzks mvkxqh nthkh gxlp vbztx fshdvz hqpfl tkjs fmgxh fdxgf zddcbth lxxm (contains shellfish)
lcvcng zmzz spg hsxpqftn gn kbgcsp nljd klpx cjdmk zddcbth grxfxfr xppzqqz hrrl cskbmx pzvb pgqxvc xlxknk hxpxh jrmr xbt mpbm pqnlq mfdv bmhn tqxcv zdmxsgp gzks tzxcmr nmdcs lxxm hcpvz fbgf gdrj gmpvdl tkrzbv lqlt ghfxfs fdzg hcrbxt tlhqvhq rzzs tftlfk gcczzsd klhrx jxkn crkvkk xgcjxkkm kskcfv jzrjtn fxzh mmpphm djbb gqb mdjr nthkh hlsvmjh hrlzp ctsl cjkrc zqb hpjnl xgtms pqpvvm jbtf (contains dairy, sesame, peanuts)
pdhv zbmcz cjdmk nmdcs tmzmhtb hrv hddxgp xrxrs ldkp tstp mxbs tftlfk bkqj jlzhrn jrmr sfdc mngvb ghfxfs kskcfv gxtlpx mfdv xstrnv tkrzbv mmdh xrmf lqlt fmgxh mbjnfc xlxknk jdnrq tgn lxxm rzzs vndglc qdjfg mvkxqh cfccb bmhn jptrc cskbmx nnhl kbgcsp kspj nvzj fpk gxlp bmgtf vcck zzgjdt hjm tkjs xjgzz pgqxvc xjdchp gcczzsd ctsl pqjk hrlzp tjbk hlktbd hpjnl jxkkxb tzxcmr qstvls nqx (contains nuts)
zddcbth xjdchp jdnrq vqdpfb jrmr ffpq lbxt dzf gxlp mngvb vxchx sdzzhnfc nptr pqjk hqpfl gzks jxkn kxrz bxdbkvtt qcrtcf mmdh zdmxsgp pqpvvm rttkb jbp kkhzrj pbt jlzhrn mpbm pjpshs xgcjxkkm grxfxfr vcck hxbp gsfqhcd vrqbct tktzx szc gn pzfmh zmzz mxbs qdjfg nnhl cskbmx vnzdgh zgg pgqxvc sfqjtk pcrmsb kvlnft tlhqvhq xstrnv tjbk tqxcv xlxknk pdhv tstp xbt fmgxh psxvf dkfvl hcrbxt xppzqqz ppbqpx pqnlq jjb kskcfv fxzh cfccb mvkxqh vbztx gmblk sbh hrrl crkvkk hlktbd bmhn kvdbxp xjgzz nqx cjdmk hmft tkrzbv ckz sbcntv nctnv (contains peanuts, soy, sesame)
grxfxfr gnc mmdh smppjj qqgjft jzrjtn jrmr gmzk sbh hjlv xrmf bpdhjc gdrj hqbn gnspg zddcbth tftlfk pdhv jxkkxb cskbmx djbb fkrkk xjdchp mxbs psxvf qnhmjbz vbztx kxrz tgn lqlt jjb nsg hcpvz vcrpl mmpphm ckz hvpx vcck ppbqpx fdzg nvql dkfvl bxdbkvtt hxbp hcrbxt spg xlxknk fblzjv tjbk tphvg nmdcs nthkh sbcntv zqb sfdc gqzzzp gn fshdvz zdmxsgp lbxt bmhn fxzh tzxcmr mbjnfc xslk jpsnjbt jxrx qdjfg zzgjdt jxkn ttmphk tqxcv jcjt hjm bmgtf pqdtkpd sfqjtk fmgxh kvlnft (contains dairy)
tgn vnzdgh nctnv jpsnjbt xlxknk nqx fxzh lqlt pstjn nptr jcjt jbtf lcvcng cjdmk djbb dkfvl hcpvz hdfqc gxlp tktzx tzxcmr sbcntv jxkkxb kspj lxxm bmgtf zbmcz tphvg qqgjft bmhn qdjfg pqjk xgcjxkkm fkrkk mvkxqh jzrjtn vndglc ppbqpx bcrjgj hlktbd hqpfl rttkb tqxcv cskbmx nj chs fblzjv mbjnfc pjpshs nljd mfdv tlhqvhq jrmr rjgfr (contains peanuts, wheat, fish)
xgtms jssvhx gcczzsd gksjpm sfdc spg gxlp psxvf fdxgf sbh chs cjdmk fxvcg xrmf lcvcng nptr vnmbz cskbmx bmhn pzfmh pqpvvm fxzh hrv pqnlq xppzqqz dkfvl pzvb vcck lbxt kvlnft fpk bxdbkvtt pgqxvc xlxknk xjgzz tkrzbv xgcjxkkm gdrj jrmr qcrtcf pqjk tkjs rtqxnnm fdzg jdnrq cfccb hsxpqftn ttmphk mhdf hlsvmjh tftlfk fmgxh vrqbct mmdh zxpnt xrxrs fblzjv cjkrc rzhb gmpvdl crkvkk gqb qstvls ldkp zgg kxrz kspj tphvg hcpvz jxkkxb (contains peanuts, wheat)
hdfqc tzxcmr jrmr jxrx ttmphk fmgxh gmblk fpk vrqbct xrmf psxvf jbp ldkp pstjn ghfxfs hjm sbcntv ffcxn mpbm bxdbkvtt xlxknk vcrpl sbh rttkb lcvcng fshdvz nmdcs pqjk lxxm qcrtcf bpdhjc tlhqvhq mbjnfc kspj jjb xslk szc dzf fdxgf fxzh tgn xvl vnzdgh cmzpv ckz nvzj fkrkk pjpshs nqfsk fvgb spg hrlzp bmgtf zgg fblzjv jptrc gggnm pgqxvc qdjfg tmzmhtb cskbmx gksjpm pcrmsb pzfmh pdhv xgcjxkkm cjdmk zbmcz zqb jcjt hqpfl nthkh kskcfv rjgfr hqbn pqnlq hlktbd (contains wheat, sesame)
jrmr jxrx hlsvmjh gsfqhcd hvpx gmpvdl lqlt nsg fmgxh jcjt sfqjtk gcczzsd xlxknk fxvcg fdzg lbxt cjkrc xrmf fxzh flvfsz nqfsk fbgf xgcjxkkm xstrnv cfjkt nmdcs jjb sbh dzf ctsl rgbj hrv mmdh hddxgp ffrxl zzgjdt bmhn ffpq vnmbz zqb hxpxh dkfvl vcck vcrpl hjm jxkkxb hsxpqftn jbtf nthkh mhdf gnc tstp bxdbkvtt mfdv qqgjft zbmcz kkhzrj tjbk gksjpm cskbmx tzxcmr rjgfr (contains peanuts, fish)
hdfqc xlxknk hmft gxlp fbgf pqnlq pqjk cjkrc gmpvdl tftlfk jbp nctnv chs hjm tmzmhtb cskbmx klhrx fblzjv gdrj hjlv vbztx mhdf rtqxnnm nsg jdnrq lqlt qdjfg mmkmt dkfvl fmgxh xslk vxchx rgbj fxzh vnmbz vqdpfb jjb mmdh jlzhrn kvlnft qqgjft zbmcz cfjkt pcrmsb hqpfl hlsvmjh tkjs tzxcmr cjdmk zgg xjgzz xgcjxkkm rjgfr jbtf dzf jrmr szc gmzk cmzpv nqx gqzzzp zddcbth tstp nqfsk pjpshs jpsnjbt zzgjdt gcczzsd sfdc gnspg nljd vndglc fkrkk mvkxqh spg xbt mfdv rzhb nj tjbk fshdvz vrqbct gggnm (contains wheat, dairy)
hqpfl hjnnd fdzg dzf knvg cmzpv gnspg sbcntv vcck grxfxfr fmgxh tftlfk rnn psxvf fxzh gksjpm bkqj nvzj vndglc sdzzhnfc nqfsk smppjj hvpx pzfmh hpjnl nnhl klhrx jzrjtn vnzdgh fpk lqlt gnc kvdbxp xjgzz bmhn dkfvl nqx mdjr mbjnfc jrmr sbh qstvls pqjk nvql xjdchp gcczzsd fbgf hlktbd lbxt hjlv cskbmx rzhb pgqxvc nthkh fblzjv ppbqpx xbt ldkp zzgjdt mmdh zqb mpbm gxlp jpsnjbt xrmf lxxm nptr zdmxsgp jssvhx jbp tzxcmr gdrj hlsvmjh ffrxl kxrz jjb bmgtf rjgfr xgtms xstrnv xlxknk (contains fish)
fdxgf fmgxh hddxgp vrqbct mpbm pdhv tzxcmr mmkmt cjdmk mhdf hjm ckz cskbmx klpx cjkrc zzgjdt fblzjv qqgjft pqpvvm fdzg xjdchp rtqxnnm gdrj xlxknk vqdpfb gzks zddcbth fshdvz ffcxn sfqjtk zmzz pzfmh mbjnfc sfdc gqzzzp sbh rttkb qdjfg xstrnv tjbk bmhn kskcfv gmpvdl jrmr rgbj lxxm ffrxl fvgb pstjn (contains nuts)
lbxt vcck bpdhjc xslk szc rnn mxbs jxrx kbgcsp pcrmsb kskcfv nvql flvfsz qcrtcf djbb rjgfr hxpxh mmdh jxkn tjbk klpx ffpq pqdtkpd jrmr xjdchp lxxm nljd qstvls cskbmx hmft zzgjdt bxdbkvtt bmhn ldkp gxtlpx tzxcmr pstjn zxpnt gmzk gn chs pzvb xgtms xlxknk hlsvmjh rzzs pbt tftlfk fxzh kkhzrj cmzpv gdrj fmgxh sfdc hcpvz jxkkxb gxlp nsg xgcjxkkm tkjs nnhl jssvhx jpsnjbt nqfsk sdzzhnfc rttkb qdjfg jdnrq (contains fish, soy, dairy)
tstp spg tftlfk pqdtkpd hrrl hdfqc xjgzz mfdv jxkkxb fxzh hxbp jssvhx bxdbkvtt jzrjtn pzfmh gnspg pqpvvm hlktbd xlxknk gqzzzp vbztx tzxcmr psxvf tlhqvhq nljd chs cskbmx ffpq nthkh flvfsz gmzk bmhn rnn rttkb knvg vnmbz mngvb sbcntv ctsl jrmr gmpvdl zgg fdzg kskcfv ttmphk pbt jptrc jdnrq qqgjft fmgxh nsg gn tktzx nmdcs sbh fvgb xrxrs ffrxl lqlt tqxcv pzvb fshdvz lbxt tmzmhtb vndglc kspj pstjn gsfqhcd hsxpqftn nptr crkvkk xslk (contains fish, sesame)
fbgf cskbmx vcck vcrpl gxlp kskcfv qnhmjbz fshdvz mpbm qcrtcf grxfxfr tzxcmr pqpvvm fxvcg tktzx fkrkk xrmf zmzz ctsl gggnm pgqxvc fpk qdjfg fmgxh cfccb nljd gqb nsg vnmbz kvlnft xlxknk zdmxsgp rttkb cjdmk jjb kspj nmdcs szc vnzdgh xbt zbmcz nthkh gqzzzp vxchx qstvls tqxcv jrmr ckz tlhqvhq lbxt hsxpqftn rgbj fxzh gmpvdl sbh hvpx pstjn vrqbct cfjkt mhdf jcjt (contains wheat)
vqdpfb ctsl pgqxvc hqbn lxxm vnmbz gdrj gnspg fblzjv tkrzbv szc ttmphk zbmcz tphvg vrqbct mbjnfc kxrz xrmf tkjs fkrkk bpdhjc lbxt hjlv zgg nthkh jrmr rgbj cjdmk xgtms bxdbkvtt pjpshs knvg hlsvmjh sfqjtk xlxknk kskcfv fxzh xjdchp mhdf vnzdgh qdjfg ldkp fmgxh tjbk hrlzp nptr ffpq mmdh sfdc jbp klhrx lcvcng gnc cskbmx zddcbth hpjnl qcrtcf rzzs rttkb cfjkt hsxpqftn hrv mvkxqh nj vndglc gggnm hdfqc pzfmh hjm bmhn fxvcg rzhb lqlt nvql nctnv xgcjxkkm (contains fish, wheat)
tkjs vcck pstjn kspj lbxt ctsl hjnnd klpx tmzmhtb nljd hpjnl qdjfg zbmcz ldkp hdfqc nnhl jzrjtn qstvls tjbk rtqxnnm fdxgf xrxrs gn spg qnhmjbz fvgb fbgf hlktbd hmft djbb jbp rzhb chs hvpx tzxcmr cjdmk gggnm gmblk ffrxl zzgjdt hqbn jjb tqxcv xjdchp gksjpm mxbs cskbmx fmgxh hjlv szc fblzjv nj klhrx kxrz nsg gzks ffcxn zdmxsgp bpdhjc jcjt tktzx tphvg cfccb kkhzrj xppzqqz hlsvmjh vrqbct hrv nmdcs bmhn dzf pgqxvc pqjk xlxknk rzzs hrrl jrmr grxfxfr (contains fish, nuts)
jrmr tkrzbv ttmphk cskbmx ldkp gzks pbt mdjr jjb dzf jdnrq tzxcmr smppjj bpdhjc gnc pqnlq gmzk jzrjtn tstp crkvkk grxfxfr gqzzzp cjdmk xstrnv szc mxbs zbmcz mngvb gnspg psxvf rzhb cmzpv qdjfg pstjn pqpvvm fmgxh tftlfk xlxknk zqb hqbn zmzz fxzh fblzjv (contains nuts, wheat, shellfish)
hxbp tstp tftlfk cskbmx hjnnd szc cfccb hsxpqftn xslk fxzh spg jxrx flvfsz fdxgf mpbm lxxm chs cfjkt bmhn jxkkxb grxfxfr jrmr xlxknk gnc rttkb bcrjgj cjdmk vbztx tzxcmr rnn kxrz fbgf xrmf gqb mfdv jpsnjbt xjdchp tmzmhtb gmblk hddxgp hrv hcpvz hjlv kspj bkqj knvg hmft fvgb mvkxqh jcjt vcck (contains peanuts, wheat, fish)
knvg pqpvvm szc nljd mmpphm gksjpm jxkkxb kkhzrj gxtlpx smppjj jcjt lqlt hlktbd xjdchp xrmf tphvg tkjs hqbn bmhn hxpxh gggnm mbjnfc chs xjgzz xstrnv kspj vqdpfb qnhmjbz nthkh fxzh fmgxh pqdtkpd hcrbxt tzxcmr klhrx hlsvmjh fdzg pjpshs nmdcs jrmr cskbmx pstjn ppbqpx fkrkk klpx gqzzzp sdzzhnfc gsfqhcd hcpvz mhdf xlxknk bmgtf vbztx zxpnt hpjnl (contains fish)
rtqxnnm jzrjtn jrmr zmzz psxvf gnspg zgg cjkrc bmhn hjlv dkfvl dzf gxlp nvql jbtf mfdv kspj smppjj kskcfv qstvls gksjpm nj fmgxh jlzhrn nmdcs cjdmk mmpphm xlxknk ctsl nvzj pgqxvc tphvg kbgcsp pjpshs djbb mvkxqh lqlt zzgjdt sdzzhnfc qdjfg vbztx vnzdgh hpjnl klhrx sbcntv zbmcz xstrnv cskbmx rnn jptrc fxvcg ffrxl pqjk vnmbz xrxrs bkqj vqdpfb crkvkk ttmphk szc tmzmhtb vndglc fblzjv pqpvvm xppzqqz qqgjft tzxcmr gmzk lbxt (contains nuts, peanuts)
xjgzz bcrjgj cjkrc hcpvz bmhn psxvf tktzx gn jbp rzzs ckz kxrz rjgfr hrlzp nsg fxzh xbt tftlfk xlxknk jrmr jxrx kbgcsp hcrbxt cjdmk jxkkxb hmft ppbqpx pqdtkpd zgg vcck chs fmgxh xgcjxkkm jcjt gnspg jlzhrn fdxgf hxpxh vbztx zxpnt cmzpv hvpx qnhmjbz ffrxl flvfsz ldkp qcrtcf xgtms cskbmx qqgjft hrrl zqb fpk pqpvvm gxlp (contains sesame, soy)
xlxknk hqpfl mdjr tzxcmr xppzqqz fxzh ffrxl hdfqc vrqbct jptrc pbt jssvhx hsxpqftn jxkn rnn mngvb bmhn cjdmk zdmxsgp hmft hxbp mmpphm ldkp nvql klpx qqgjft spg tlhqvhq zgg fvgb gxtlpx fshdvz zbmcz pstjn nqx gxlp pcrmsb lqlt mmkmt hjlv bxdbkvtt gnspg jpsnjbt jrmr kkhzrj cfjkt jdnrq fmgxh rjgfr pgqxvc tjbk szc (contains soy, sesame, shellfish)
rtqxnnm xlxknk jcjt vrqbct smppjj pqjk gnc zqb gmpvdl kbgcsp zxpnt ffrxl kskcfv jjb gmzk lqlt mmkmt szc tzxcmr mxbs zgg pstjn vxchx lbxt tftlfk fxvcg mvkxqh vcrpl bmhn mdjr nvql sbh xrmf mpbm ffpq hxbp tkrzbv jbtf kxrz xjgzz fxzh zbmcz cmzpv hjlv hcrbxt sfdc hrlzp xppzqqz flvfsz dkfvl jrmr mmdh cskbmx zddcbth jxkn lcvcng gqb mhdf cjdmk hpjnl (contains shellfish, dairy)
hqpfl vbztx hrrl cmzpv jptrc mbjnfc fkrkk pqdtkpd nthkh kskcfv cjdmk lxxm nmdcs mpbm gnc mmdh klhrx ghfxfs gcczzsd hcrbxt pzvb gmblk kbgcsp lcvcng vcck xppzqqz lqlt jrmr hjlv rttkb pqnlq fxvcg ffcxn mxbs kvlnft xjgzz tzxcmr kvdbxp bmhn hqbn zddcbth sbh bkqj klpx ldkp zmzz jpsnjbt pqpvvm fmgxh mfdv tkrzbv sbcntv gn tkjs cskbmx jdnrq xgtms nj hrv smppjj tjbk mngvb gxlp xlxknk xgcjxkkm kxrz xrmf hdfqc tqxcv cjkrc sfdc szc jzrjtn (contains soy)
xrxrs kvdbxp jlzhrn klhrx jjb jssvhx tftlfk vbztx tkrzbv qnhmjbz zddcbth kbgcsp cjkrc rjgfr hlsvmjh xvl flvfsz lcvcng jrmr vqdpfb szc gcczzsd pjpshs cfjkt rzhb tzxcmr jxkn fblzjv lqlt xppzqqz jcjt nctnv vrqbct xgtms nptr gqzzzp mdjr vcck kxrz psxvf fxzh hcrbxt hddxgp ctsl sfqjtk bxdbkvtt fkrkk mhdf rgbj mngvb xlxknk nvzj xbt rtqxnnm chs rnn cskbmx sbh xstrnv mpbm pqnlq hjnnd tgn hrlzp gksjpm nvql xjgzz vndglc fpk cjdmk qqgjft fdxgf fvgb rttkb fshdvz bmhn (contains sesame, fish, nuts)
lxxm mmdh gxtlpx dzf xstrnv gsfqhcd jpsnjbt jbp jjb lbxt cskbmx rzzs bpdhjc fshdvz gnc gzks jxkkxb nljd mdjr hxbp zdmxsgp hrlzp cfjkt tkjs jssvhx pqjk lcvcng cjdmk chs lqlt kspj bmhn gmzk hdfqc hlktbd tjbk gdrj gxlp szc crkvkk gmblk rgbj xlxknk ctsl zxpnt zgg jxkn pqnlq gksjpm tmzmhtb hpjnl tqxcv nvzj ghfxfs rzhb gn jrmr vnzdgh zddcbth hrv hrrl nctnv fxzh gnspg xrxrs fmgxh mngvb xgcjxkkm fblzjv vbztx (contains peanuts)
psxvf dzf pqpvvm mmdh pstjn nnhl gcczzsd tlhqvhq jxrx vcck tstp hqpfl rnn hxbp cfccb xlxknk hsxpqftn xrmf nljd gxtlpx gn fmgxh cjdmk tjbk jjb gggnm jrmr mfdv bxdbkvtt lbxt gnspg nqfsk tkrzbv kbgcsp pqjk bmgtf cskbmx zmzz pjpshs gzks tzxcmr fxzh hjnnd grxfxfr vqdpfb tgn gsfqhcd xvl hvpx (contains wheat)
ldkp djbb cjdmk pqpvvm pgqxvc jptrc bxdbkvtt pqdtkpd fxzh lxxm nj mngvb zxpnt cmzpv pqjk jbp vnzdgh pstjn crkvkk sfdc rtqxnnm kbgcsp ffpq nmdcs fxvcg jzrjtn gdrj kxrz sdzzhnfc nvql cskbmx jrmr xstrnv lbxt vnmbz tqxcv xgtms gggnm fmgxh rzzs fdzg xppzqqz xvl nctnv nsg qstvls jlzhrn ffcxn kspj kvdbxp rjgfr vqdpfb bcrjgj gcczzsd hsxpqftn pzvb bmhn chs mpbm pqnlq hddxgp tzxcmr gmzk mfdv tstp jxkn hrrl gmblk ghfxfs pzfmh bmgtf tkjs gksjpm (contains wheat, sesame)
mhdf tmzmhtb chs tzxcmr zddcbth pgqxvc gzks vqdpfb qstvls pstjn nctnv xppzqqz bmhn tftlfk tkjs hjm fvgb nj fmgxh tlhqvhq gnspg zxpnt xrxrs flvfsz cjdmk kvdbxp rnn knvg jbp crkvkk hrrl mvkxqh bcrjgj vcck jzrjtn zdmxsgp mmkmt cfjkt kspj gnc xgcjxkkm tkrzbv xjdchp jrmr lbxt hsxpqftn nljd vnmbz xlxknk tjbk cjkrc hrlzp pzvb cskbmx mmdh kbgcsp smppjj klhrx fpk nqfsk mngvb nsg (contains soy)
spg nsg gdrj tgn qdjfg rgbj nctnv nvzj ctsl zmzz qstvls mfdv kvlnft vcck tzxcmr tmzmhtb hlktbd mhdf gmzk tktzx vcrpl grxfxfr hsxpqftn pzvb kspj nqx hcrbxt fxzh nvql mmpphm vnmbz jlzhrn cjdmk ppbqpx hrrl bxdbkvtt psxvf fmgxh tftlfk jptrc fpk tkrzbv tjbk nptr pcrmsb gxlp cskbmx rjgfr gn nmdcs xstrnv jxkkxb xlxknk fdxgf lbxt gzks gksjpm hdfqc hxpxh jdnrq xppzqqz nthkh bmhn fkrkk mmdh cfjkt (contains nuts, soy)
nvql fshdvz rzzs fblzjv jrmr fmgxh gxlp fxzh cskbmx rtqxnnm vqdpfb mhdf pzfmh xppzqqz rgbj ldkp xvl grxfxfr nsg hmft lbxt hlsvmjh gqb rzhb bpdhjc nnhl qnhmjbz cfjkt bmhn ffrxl sdzzhnfc gn mvkxqh gmpvdl pstjn pcrmsb jxkkxb hqpfl gzks gksjpm cjdmk ghfxfs dzf nqx tgn tzxcmr hjnnd xrxrs xgcjxkkm zbmcz jzrjtn (contains fish)
tqxcv pgqxvc hxpxh vcrpl gn mbjnfc fmgxh gmpvdl gxtlpx hjm qnhmjbz nvql klhrx cjdmk bxdbkvtt vcck sdzzhnfc bpdhjc pzfmh jrmr tkrzbv cfjkt qdjfg nctnv rgbj nqx hmft sfdc fdxgf fxzh cskbmx crkvkk dkfvl nj mmpphm rnn hddxgp kbgcsp xrmf rtqxnnm hjnnd vqdpfb gnc jlzhrn pqpvvm jzrjtn hqbn gcczzsd lxxm pqjk hqpfl hcpvz jbp pjpshs tphvg rjgfr mngvb zxpnt tzxcmr gxlp fkrkk ctsl bmhn vnmbz hjlv mvkxqh (contains soy, wheat)
qqgjft xvl jlzhrn pstjn klhrx tstp zdmxsgp gksjpm vbztx cskbmx fblzjv jrmr szc nljd zqb gmpvdl sbh mngvb tjbk klpx cfccb ffrxl nctnv fvgb hddxgp gzks jzrjtn tgn dzf mmkmt zgg zzgjdt gmblk jdnrq rzzs cmzpv zmzz sbcntv nqx hdfqc jssvhx rttkb nvql jxrx xslk hcpvz pqjk hsxpqftn mmdh tftlfk pbt hpjnl pdhv nj rjgfr gmzk jcjt nsg spg gdrj cfjkt xbt cjdmk xjgzz zxpnt hmft hlktbd hqpfl tzxcmr jxkn xlxknk lbxt mhdf pzvb tkjs pzfmh fmgxh hrrl crkvkk fpk fshdvz fxzh mbjnfc gnspg mxbs xgcjxkkm pqpvvm vqdpfb ffcxn ffpq (contains fish, shellfish)
nmdcs lbxt rtqxnnm kspj xlxknk dkfvl bmhn jjb sfdc xbt bpdhjc fkrkk pcrmsb bmgtf qdjfg qcrtcf crkvkk jdnrq smppjj nvzj nj jxrx tzxcmr vqdpfb sdzzhnfc fmgxh gksjpm gmpvdl ffcxn ttmphk sbcntv bxdbkvtt hqpfl cjdmk mvkxqh ldkp jxkn gqzzzp cskbmx jrmr gggnm fdxgf ffpq ghfxfs flvfsz nqfsk mbjnfc xstrnv jbp rnn zzgjdt nthkh hqbn fdzg (contains sesame, dairy)
ldkp nj zzgjdt sdzzhnfc hxpxh nvzj nvql nsg pqdtkpd tzxcmr vrqbct rgbj gxlp lbxt gqzzzp fvgb xlxknk sbh pcrmsb mmdh rjgfr gnspg jrmr xslk knvg zddcbth grxfxfr tjbk qdjfg jxkn zbmcz tmzmhtb fmgxh xgtms gxtlpx lcvcng spg zdmxsgp ffpq kspj hsxpqftn chs pjpshs gzks jpsnjbt mhdf ghfxfs tphvg sfqjtk bmgtf jbp tstp hqbn zqb mngvb hjm cjdmk tktzx cskbmx hmft hjlv nqx kskcfv fdxgf vqdpfb bpdhjc jptrc crkvkk djbb gmpvdl klpx fxzh gmblk gn fblzjv (contains shellfish, nuts, dairy)
flvfsz nnhl hxpxh qnhmjbz gqzzzp gsfqhcd knvg lxxm hjnnd jdnrq zgg kbgcsp nsg vndglc cmzpv zbmcz vrqbct ckz pqnlq cskbmx tftlfk xvl hqbn jssvhx lbxt hrv tzxcmr hpjnl sdzzhnfc rttkb pjpshs sbh fshdvz ffpq pqpvvm gxtlpx kskcfv fmgxh bpdhjc jlzhrn xrxrs psxvf zzgjdt hxbp cfccb nptr xgcjxkkm hcrbxt xslk cjdmk gnspg jrmr xlxknk bmhn mpbm hmft jxkkxb lcvcng tmzmhtb mmpphm xstrnv pzfmh (contains nuts)
tgn psxvf pzvb nqx fxvcg sbcntv cskbmx hxpxh hdfqc qdjfg cfjkt nqfsk ckz zdmxsgp bxdbkvtt ldkp vndglc rjgfr fvgb gxlp gnc fbgf zgg mpbm ctsl rtqxnnm jzrjtn gksjpm xjgzz jssvhx tjbk ghfxfs mdjr xlxknk pstjn cjdmk hxbp sfdc jrmr jcjt fxzh xslk jpsnjbt bmgtf mngvb qnhmjbz jjb qstvls nljd pqnlq vbztx pqdtkpd kbgcsp tkjs bmhn gzks jlzhrn hmft gmpvdl hjnnd lcvcng mvkxqh fmgxh knvg zxpnt tkrzbv mmkmt kspj (contains dairy, nuts)
bmhn rjgfr gdrj mmpphm nj fvgb pqjk cfjkt fxzh hvpx xjdchp cjkrc gnc sfqjtk pqpvvm tmzmhtb vcrpl pdhv gsfqhcd ffcxn xlxknk hjm qqgjft sbcntv nvzj bxdbkvtt vcck fpk kvdbxp tftlfk tjbk rttkb kskcfv fkrkk zzgjdt hjnnd hpjnl bmgtf gxtlpx xrmf hqpfl hmft tlhqvhq jrmr cfccb pgqxvc cskbmx tzxcmr mhdf cjdmk nnhl gmblk (contains wheat, peanuts, sesame)
zmzz chs xlxknk pbt jbp gsfqhcd cskbmx lbxt gggnm nqfsk bmhn zbmcz vcrpl jrmr hsxpqftn sdzzhnfc nmdcs hcrbxt gmzk gmblk fmgxh jxkkxb fpk gdrj gxlp cfccb sbcntv hxpxh zddcbth vnmbz pqnlq xrxrs crkvkk xvl mmdh xppzqqz gcczzsd ldkp xstrnv jssvhx kspj qqgjft bcrjgj zdmxsgp fvgb cfjkt fdxgf xbt kvlnft hjnnd bxdbkvtt hcpvz hmft tzxcmr fxzh tkrzbv gnc rzhb mmkmt gmpvdl (contains shellfish, fish, soy)
vnzdgh fvgb nptr hjnnd zdmxsgp mmdh fxzh nthkh xlxknk tzxcmr sbcntv vcck nvzj gmblk grxfxfr bmhn chs qqgjft xstrnv tphvg mvkxqh djbb qcrtcf mmkmt jcjt lqlt hlktbd xjgzz tjbk hvpx fblzjv vxchx jzrjtn kbgcsp xgcjxkkm nqx pqdtkpd vcrpl spg pstjn ffcxn hrrl cjdmk jjb cskbmx crkvkk gqb hqbn hrv zgg fmgxh jxkn (contains peanuts)
tjbk nqfsk cjdmk xrmf cmzpv tstp fmgxh gnc bmgtf kskcfv jcjt ghfxfs jxkn mxbs gksjpm gggnm tftlfk chs lxxm cskbmx hdfqc qstvls gsfqhcd fshdvz gn hjlv tkrzbv hlktbd fxzh vcck bxdbkvtt nsg hrrl qdjfg pqnlq qcrtcf sfqjtk sfdc nvql tmzmhtb gqb bmhn jbp pjpshs xlxknk dkfvl gxlp jrmr kvlnft klhrx rjgfr spg pzfmh nj hxbp pqdtkpd jjb vbztx (contains peanuts, dairy, soy)
hxbp sbh kspj mmkmt mvkxqh mpbm tzxcmr crkvkk mdjr zdmxsgp qnhmjbz gmblk hsxpqftn mmpphm xstrnv mmdh jzrjtn gksjpm zqb nptr szc cskbmx gmzk fxzh jxkn nljd mngvb lqlt kbgcsp bcrjgj cjdmk tftlfk kxrz hdfqc tgn nj ckz vndglc jptrc bmgtf vqdpfb chs xgcjxkkm gqzzzp fshdvz hcpvz mfdv nqfsk smppjj hqbn sbcntv fpk xjdchp gnc qcrtcf ctsl mhdf mbjnfc fmgxh mxbs pqpvvm gqb cjkrc sfdc hqpfl bmhn zgg jrmr xvl (contains fish, shellfish)";
