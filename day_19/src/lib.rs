use std::ops::RangeInclusive;

use prelude::*;

use range_set::RangeSet;

#[derive(Debug)]
enum Category {
    X,
    M,
    A,
    S,
}
use Category::*;

impl From<&str> for Category {
    fn from(value: &str) -> Self {
        match value {
            "x" => X,
            "m" => M,
            "a" => A,
            "s" => S,
            _ => panic!("invalid category {value:?}"),
        }
    }
}

#[derive(Debug)]
enum Criterion {
    Unconditional,
    Gt(Category, u64),
    Lt(Category, u64),
}
use Criterion::*;

#[derive(Debug)]
enum Disposition {
    Next(String),
    Accept,
    Reject,
}
use Disposition::*;

impl From<&str> for Disposition {
    fn from(value: &str) -> Self {
        match value {
            "A" => Accept,
            "R" => Reject,
            _ => Next(value.to_owned()),
        }
    }
}

type Rule = (Criterion, Disposition);

#[derive(Clone, Debug)]
struct Fourple<T> {
    x: T,
    m: T,
    a: T,
    s: T,
}

impl<T: Clone> Fourple<T> {
    fn get(&self, category: &Category) -> &T {
        match category {
            X => &self.x,
            M => &self.m,
            A => &self.a,
            S => &self.s,
        }
    }

    fn get_mut(&mut self, category: &Category) -> &mut T {
        match category {
            X => &mut self.x,
            M => &mut self.m,
            A => &mut self.a,
            S => &mut self.s,
        }
    }
}

type Rating = Fourple<u64>;
type AttributeRange = RangeSet<[RangeInclusive<u64>; 10]>;

pub struct Solution {
    rules: HashMap<String, Vec<Rule>>,
    ratings: Vec<Rating>,
}

lazy_static::lazy_static! {
    static ref RATING: Regex = Regex::new(r"^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}$").unwrap();
}

impl Day for Solution {
    fn new(input: &str) -> Self {
        let (top, bottom) = input
            .split_once("\n\n")
            .expect("there must be a blank line in your input");

        let rules = top
            .lines()
            .map(|line| {
                let (name, rest) = line.split_once('{').expect("could not find a {");
                let rest = rest.strip_suffix('}').expect("could not find a }");
                let rules = rest
                    .split(',')
                    .map(|rule| -> Rule {
                        if let Some((l, r)) = rule.split_once(':') {
                            if let Some((category, count)) = l.split_once('<') {
                                (
                                    Lt(category.into(), count.parse().expect("bad integer")),
                                    r.into(),
                                )
                            } else if let Some((category, count)) = l.split_once('>') {
                                (
                                    Gt(category.into(), count.parse().expect("bad integer")),
                                    r.into(),
                                )
                            } else {
                                panic!("input was neither > nor <");
                            }
                        } else {
                            (Unconditional, rule.into())
                        }
                    })
                    .collect();

                (name.to_owned(), rules)
            })
            .collect();

        let ratings = bottom
            .lines()
            .map(|line| {
                let captures = RATING.captures(line).expect("ratings don't match regex");
                let x = captures
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("bad integer");
                let m = captures
                    .get(2)
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("bad integer");
                let a = captures
                    .get(3)
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("bad integer");
                let s = captures
                    .get(4)
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("bad integer");

                Rating { x, m, a, s }
            })
            .collect();

        Solution { rules, ratings }
    }

    fn part1(&self) -> anyhow::Result<u64> {
        Ok(self
            .ratings
            .iter()
            .filter_map(|r| match self.is_accepted(r) {
                Err(e) => Some(Err(e)),
                Ok(false) => {
                    log::debug!("rejected {r:?}");
                    None
                }
                Ok(true) => {
                    log::debug!("accepted {r:?}");
                    Some(Ok(r.x + r.m + r.a + r.s))
                }
            })
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .sum())
    }

    fn part2(&self) -> anyhow::Result<u64> {
        self.count_accepted(
            "in",
            Fourple {
                x: RangeSet::from_ranges(&[1..=4000]),
                m: RangeSet::from_ranges(&[1..=4000]),
                a: RangeSet::from_ranges(&[1..=4000]),
                s: RangeSet::from_ranges(&[1..=4000]),
            },
        )
    }
}

impl Solution {
    fn is_accepted(&self, rating: &Rating) -> anyhow::Result<bool> {
        let mut wf_name = "in";
        log::debug!("evaluating {rating:?}");

        loop {
            log::debug!("looking at workflow {wf_name:?}");
            let workflow = self
                .rules
                .get(wf_name)
                .ok_or_else(|| anyhow::anyhow!("could not find workflow {wf_name:?}"))?;
            log::debug!("workflow is {workflow:?}");

            let mut chosen = None;
            for (criterion, disposition) in workflow {
                match criterion {
                    Unconditional => {
                        chosen = Some(disposition);
                        break;
                    }
                    Gt(c, target) => {
                        if rating.get(c) > target {
                            chosen = Some(disposition);
                            break;
                        }
                    }
                    Lt(c, target) => {
                        if rating.get(c) < target {
                            chosen = Some(disposition);
                            break;
                        }
                    }
                }
            }

            match chosen {
                Some(Accept) => return Ok(true),
                Some(Reject) => return Ok(false),
                Some(Next(name)) => wf_name = name,
                None => anyhow::bail!("none of the rules matched"),
            }
        }
    }

    fn count_accepted(
        &self,
        state: &str,
        mut possibilities: Fourple<AttributeRange>,
    ) -> anyhow::Result<u64> {
        let mut res = 0;

        let next = self
            .rules
            .get(state)
            .ok_or_else(|| anyhow::anyhow!("could not find state {state:?}"))?;

        for (criterion, disposition) in next {
            if [
                &possibilities.x,
                &possibilities.m,
                &possibilities.a,
                &possibilities.s,
            ]
            .into_iter()
            .any(|r| r.is_empty())
            {
                // no amount of set-intersections is ever going to bring this back...
                break;
            }

            let mut recur_possibilities = possibilities.clone();

            match criterion {
                Lt(cat, target) => {
                    let recur_ranges = recur_possibilities.get_mut(cat);
                    let continue_ranges = possibilities.get_mut(cat);
                    let inverse = shrink_range(recur_ranges, 0..=(target - 1));
                    *continue_ranges = inverse;
                }
                Gt(cat, target) => {
                    let recur_ranges = recur_possibilities.get_mut(cat);
                    let continue_ranges = possibilities.get_mut(cat);
                    let inverse = shrink_range(recur_ranges, (target + 1)..=u64::MAX);
                    *continue_ranges = inverse;
                }
                Unconditional => {
                    // we will recur on everything that's possible so far, and then we're done.  Set
                    // one (any one!) of the attribute ranges to a completely empty one.
                    let mut empty = AttributeRange::new();
                    std::mem::swap(&mut possibilities.x, &mut empty);
                }
            }

            match disposition {
                Accept => {
                    res += count(&recur_possibilities);
                }
                Reject => {
                    // do nothing, but let the shrunken range continue down the list of rules
                }
                Next(s) => {
                    res += self.count_accepted(s, recur_possibilities)?;
                }
            }
        }

        Ok(res)
    }
}

// Intersects ranges with intersect_with, and returns ranges & !intersect_with
fn shrink_range(
    ranges: &mut AttributeRange,
    intersect_with: RangeInclusive<u64>,
) -> AttributeRange {
    // since there's no intersect() function in RangeSet, but doing inserts and removals will *return* the ranges
    let intersected = ranges.remove_range(intersect_with);
    if let Some(mut r) = intersected {
        // put the intersection where we expected, and return the inverse of the intersection
        std::mem::swap(&mut r, ranges);
        r
    } else {
        // there was no intersection, so ranges becomes empty and everything that *was* in ranges should be returned
        let mut empty = AttributeRange::new();
        std::mem::swap(&mut empty, ranges);
        empty // which is not empty anymore
    }
}

fn count(ranges: &Fourple<AttributeRange>) -> u64 {
    [&ranges.x, &ranges.m, &ranges.a, &ranges.s]
        .into_iter()
        .map(|r| {
            r.as_ref()
                .iter()
                .map(|std_range| std_range.end() - std_range.start() + 1)
                .sum::<u64>()
        })
        .product()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let solution = Solution::new(EXAMPLE);
        assert_eq!(19114, solution.part1().unwrap());
        assert_eq!(167409079868000, solution.part2().unwrap());
    }

    #[test]
    fn personal_input() {
        let solution = Solution::new(INPUT);
        assert_eq!(287054, solution.part1().unwrap());
        assert_eq!(131619440296497, solution.part2().unwrap());
    }

    static EXAMPLE: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    static INPUT: &str = "pbm{s>1594:R,a>50:A,A}
gng{a>418:sxj,m<3591:pns,x>3049:crp,dxn}
gq{x<1322:A,a<3463:fz,A}
szd{m>2793:A,R}
kgj{m>1255:R,m>830:A,R}
vc{m<633:A,R}
xf{x<2201:th,s<3427:R,m<2585:kcn,gbq}
lk{s<2220:nk,s<3023:jb,fts}
slm{a>288:R,R}
xfn{s<3350:ztv,a>3455:cpd,a>3349:qn,R}
pk{x>520:R,R}
dfz{a<3400:A,x<1012:qh,x>1270:vfs,A}
chf{x<3835:fcp,s<2347:br,ps}
zr{s<1583:kv,m>3345:qrx,zdf}
bch{m>1172:bnx,a<353:rdj,csc}
qmq{s<3300:R,x>2494:A,s<3435:A,A}
srj{a>3402:A,R}
mvq{s>2892:jgd,m<1855:xpf,x<639:lkp,pjf}
lxj{a<2291:A,x>1072:A,A}
bf{a>868:zjl,s>772:cj,a<779:pln,xpz}
nvq{x>765:A,kcl}
tld{m<1404:hb,xvf}
ddj{m<3263:A,m<3606:ks,bjf}
pq{s<1221:A,a>2810:A,x>1955:A,R}
tl{s<2902:nx,nzb}
rn{m<3138:R,a>598:R,R}
cf{s<1327:R,m>604:A,A}
rzq{x>2794:R,s<2190:A,R}
pxg{m>1575:R,x>538:lp,m<969:A,mg}
fb{s<2640:R,x<974:A,A}
zt{m>896:xpj,mp}
jjh{x>3674:A,A}
fzp{a>2549:dlq,a<2179:nqs,x<1631:lcf,fgg}
vf{x>1525:R,s<904:kjq,hg}
qzk{s>3108:R,x<1390:R,s>3079:A,R}
bmf{x<3362:R,m>2095:A,s>3460:A,kx}
dgj{x>3489:bch,jg}
cpd{a>3510:R,x>1892:A,x<1774:A,A}
fsr{m<1942:R,m>2181:xz,m<2090:zn,A}
cn{a<2031:A,m<1216:A,A}
fq{s<2556:bdf,a>485:kz,m>3754:mnr,sx}
kcn{s<3674:R,A}
vp{s<3448:A,x<3142:R,R}
cr{m>3771:R,A}
fx{x>878:A,a<3875:R,A}
tng{a<3833:np,A}
nhl{s>2837:A,a>1802:sxz,xm}
kq{s>3339:A,a<2644:A,A}
fjx{s<492:R,a>2666:R,A}
sn{x<1394:A,s<3790:A,R}
grd{x<2647:xf,vt}
mmr{m>2339:vbs,bbr}
qh{s>2871:A,s>2554:R,A}
bq{s>1785:A,s<846:A,A}
zjn{s<2790:R,a>2032:R,R}
xg{x>2244:A,R}
vzq{s<3654:kq,R}
nh{m<2332:R,x<1451:R,A}
xqr{m<1663:pfz,bhg}
hs{a<911:A,x<3083:R,a<916:A,A}
xpz{a>815:dlx,sp}
bk{s>3732:mhh,x>1506:hqp,a>258:pgt,fhz}
zmx{a<1685:zx,nvq}
hnp{s>2478:zk,x>541:qr,R}
xh{a<3354:kvl,m<2448:mrm,s>3411:qdb,dfz}
rhx{a<268:A,R}
tgf{a<157:slq,a>180:frs,a<169:R,ctl}
dlx{x<3125:A,A}
qq{a<2678:A,a<2788:xjg,m>2393:gbb,km}
zxf{s<2870:mmp,a<1196:A,a<1373:R,ljz}
sjl{a<1170:A,a>1361:R,R}
sj{m<1060:A,R}
mbf{x<1731:rf,a<627:fmh,qgs}
jm{a<1509:sqs,m<2503:zmx,m<3114:mh,dgk}
dd{m>506:R,s<3312:A,A}
sjv{s>1378:A,m>557:A,s>1034:R,R}
mxq{m<3569:R,R}
mhh{s>3902:nh,a<269:A,R}
xn{x<3273:rn,x>3684:xll,a<429:A,R}
gnd{s>2726:A,R}
lz{s>2909:qt,m<2463:qp,R}
bcr{s>3259:zd,a<614:qzk,a>851:hgq,nt}
jbk{m>421:R,a>2107:A,A}
ng{m>1074:R,a<3057:R,s<3614:A,A}
vh{x<2106:jm,m>2324:nnc,s>2548:mpf,tms}
gc{m>2708:rzq,m<2622:R,R}
bhg{x>2294:A,x<2226:R,a<1455:R,R}
lx{s<554:vfz,x>2627:ccd,A}
tn{m>1506:dn,x>1815:hm,bb}
krr{s>1460:A,s>626:R,a<595:A,A}
dgk{s<1450:ms,rkk}
vt{s<2925:zjh,m<1594:A,a<2495:A,R}
hqp{a>267:R,m<1617:pkm,s<3580:kbg,thb}
fdc{s<3718:R,x>1491:R,x>505:R,A}
tcr{m<1364:R,A}
qp{s>2703:A,A}
pf{x>316:A,A}
hg{m<3748:A,s<1450:R,R}
qvt{x>3247:fmm,s<2674:gc,a<394:ccr,vx}
glx{m>2808:rx,s<509:A,A}
lcz{m>3193:kb,A}
hgz{x<1487:R,x<1857:rj,x>2009:R,tk}
rlf{a>698:R,a>674:R,a<655:R,A}
fl{m<884:A,m<1164:R,A}
mdv{x<3773:qqq,fq}
vtd{a<2466:R,x>320:A,R}
xs{s>1390:A,s>1185:rhx,x>3223:dqb,A}
smj{m>2124:xrj,lb}
jkh{m>3508:vf,qx}
ljz{x>2952:R,s>3130:A,A}
cpx{m<366:sz,cf}
tvc{a<2013:R,m>2852:R,R}
tb{s<1319:R,a<2646:A,sjj}
hsz{a<914:A,A}
zhd{s>3791:R,A}
mv{x>2183:nxt,m<2134:nf,x<1219:vvl,R}
nkd{s>3343:A,s<2878:R,m>1115:R,A}
np{m>2002:R,m>928:R,A}
ssl{s>2435:R,m>496:A,a>635:A,A}
bdf{s>2106:A,a>592:R,a>226:A,A}
plz{a>1612:R,m<840:A,R}
ts{m>3676:R,a<1793:A,m<3342:A,A}
kp{a<119:pbm,s<2380:A,a>174:tj,R}
mb{s<2956:vd,x>3089:bm,a>3569:pd,bsg}
kbg{s>3534:A,R}
hl{a>2659:R,s>3781:A,R}
rpd{s<2643:vxs,s<3339:A,A}
zgb{a<2718:A,R}
xd{m>1944:R,a>2108:A,m<1857:A,R}
kfl{x>2369:A,s>2491:A,a<3694:R,R}
nd{a>139:A,x<3427:A,s>917:A,A}
zjh{a<2458:A,x>3231:A,a>2517:A,A}
jk{s<2917:sl,tng}
css{x<2505:gxk,s<1688:A,R}
fp{a>432:R,R}
qcd{a>322:A,R}
nn{x>876:R,m>3389:R,A}
xpd{x<2892:xdd,s<399:R,xtv}
kr{x<2716:llp,m>1060:cx,s>1756:bqc,A}
xq{a<236:A,A}
mhr{s>2460:A,a<1262:mjd,R}
vg{x>2940:nkd,m>998:xl,pdb}
khj{s<2214:A,R}
rmc{m<1890:zvb,m>2627:bg,m<2265:rg,lz}
jgd{s<3407:A,s<3620:R,R}
zdf{m<2899:qvt,lmn}
fz{x<1619:R,s<393:A,m<2444:R,R}
gm{a<1613:R,a<1631:A,s<1103:A,A}
dlq{a>2851:fpk,s<3122:spn,ls}
qr{m<2148:R,m<3214:R,a>233:A,R}
gbq{s<3798:R,A}
hgq{m<654:A,s<3136:A,R}
jbl{x>2470:R,xb}
gpn{s>1506:rv,R}
fmh{s<2599:rc,a<221:mtt,m<1637:R,blr}
dsn{a<1543:mrs,a>1723:lfk,plz}
pjf{m<3024:cfb,m>3475:R,x<1058:A,fgf}
qg{x<3045:R,ghb}
tt{x>530:A,m<921:R,x<214:R,A}
sch{x<992:A,a>2703:A,x<1755:R,A}
npp{a<3629:R,a<3817:A,m>2613:R,kgj}
lsq{s<2199:R,A}
rtm{a>736:dfv,s<1428:pg,m<1328:ss,skb}
md{a>3394:A,a>3370:R,m>3470:A,A}
bn{x<2866:glc,m<1057:A,a>1688:A,btv}
mjd{m<3526:R,A}
plr{x<3907:R,m<870:A,s>1374:A,R}
rdh{s>2837:A,a<2940:A,A}
hm{x>3054:cd,tbc}
xt{s<3183:A,a<324:R,x<2610:A,R}
mkz{s>317:R,A}
gn{s<3396:A,s<3439:A,R}
rz{m>1170:A,s<602:R,A}
jps{m<1001:mrx,bdj}
cj{s<1106:mmg,a>724:sdc,s>1418:A,mkj}
qt{a<2271:R,m>2440:A,s>3514:R,R}
bm{x>3438:pxr,x>3304:bmf,x>3191:pl,ff}
mq{s<2901:A,ng}
xnv{a<693:jmh,a>850:ljc,s>3699:px,jdj}
rxn{a<2742:A,R}
gk{m<177:R,a<292:R,R}
zdz{s<3617:dnj,R}
ss{s<1985:bht,R}
vpf{x>2805:A,s<3584:ck,rcj}
nfq{m>2744:A,A}
xm{m>1267:A,a>1753:R,x>2743:A,A}
rv{a>1328:A,a<1163:R,A}
qtm{a<197:A,a<282:R,s<2644:R,A}
bcx{s>3302:nxv,bnv}
xrj{x<701:rm,s<3364:lxj,s>3698:jq,dq}
fd{a>2673:R,m<699:gnd,s>2632:mgr,cdr}
qv{s<2318:kl,a>3277:ztb,fzp}
rdj{a<213:kp,x<3803:slm,fg}
zf{x<1784:krr,R}
dhq{m<951:zjn,frp}
lfk{m>1198:A,m<448:A,m<838:R,R}
kb{s>2062:A,m>3284:A,A}
pcq{s<1351:A,R}
xjs{s>3771:R,A}
mkj{a<672:A,R}
dqb{a<259:R,s<1080:R,R}
qz{s<469:R,R}
fh{x>982:R,R}
sxz{a>1830:R,x<3078:A,m<1003:A,R}
bb{x<1048:xtl,m<584:nnn,tb}
cm{a<3469:A,R}
jpj{x<2616:R,A}
flv{s>507:A,a>587:R,a<384:R,A}
zhh{x>3035:R,R}
sl{s>2673:R,s<2451:A,A}
hdk{x>1235:A,x<541:R,R}
mrm{a>3430:gt,s<3283:R,s>3563:xhs,fh}
qhp{m<2737:A,x>1253:R,m>2912:A,dt}
jbm{x>689:R,A}
nr{s>1331:A,A}
jq{m>2816:R,s<3890:R,R}
pxr{s>3342:R,m>1821:A,snq}
dh{x<2709:flv,m<1318:tv,R}
vkf{s>1791:R,R}
kv{m>3372:vjv,a>639:bf,qjv}
ztv{a>3462:A,R}
rq{m<869:R,a<1914:R,R}
vx{s>3516:smn,R}
psn{x>3156:A,s<3651:A,s<3866:A,A}
jx{m>2365:qmq,m<1409:tx,a<3802:ds,A}
pfz{m>1347:A,A}
bbr{x>722:sj,R}
qht{a<227:A,A}
vj{x<3228:R,R}
vr{x<1293:A,a>2811:R,x>1654:A,R}
xvf{m>2479:A,A}
tj{m<547:A,a>193:R,m>910:A,R}
xjg{m<2805:A,R}
rj{a>1303:A,a>1174:A,a<1099:R,R}
mp{a>1345:A,A}
cdr{s<2450:R,a<2606:A,R}
lm{s<1619:pq,jtc}
xlz{m>1361:R,R}
mnr{s>3320:R,a>174:R,x<3885:A,R}
qn{m>2509:A,a>3388:R,A}
qcx{s<578:A,x>3758:A,cb}
qb{a>2747:A,m<717:A,x<3283:A,A}
tgj{x<3265:R,R}
df{x<940:cn,m<917:R,m>1727:A,sn}
pns{m>3484:qtm,a<249:A,m>3434:pqq,htm}
ph{a>604:gcm,m<1718:khj,s>2115:A,R}
cfb{a>3586:R,A}
xzs{s<3577:jbl,x<2235:df,a>1993:dzc,lc}
mxh{m>1339:R,a<2660:A,s<238:A,A}
lc{x>2980:R,rq}
mhd{s>1555:pn,pmr}
lcv{m<248:R,m>382:ssl,ljj}
kd{m<750:zxm,m<1077:R,pk}
xpf{m<1036:R,jkv}
zb{s<1413:R,m>3150:R,A}
sb{a<801:vmr,s<2880:zhh,s>3354:A,R}
kbv{s>1546:R,R}
lmn{s<2969:lcz,xn}
qd{s<3395:A,s>3762:A,x<2840:R,A}
fg{m>548:plr,m>326:A,gk}
pnl{m<1196:sr,a<3504:srj,a>3617:R,vkf}
lj{s<3621:R,R}
fcp{s>2516:A,a<651:R,A}
ms{s<805:R,a<1653:cl,a>1755:ts,R}
xb{a>2025:A,R}
bv{a<2535:xpd,a>2801:pm,x<2760:rs,sfz}
dp{x<224:R,m>2616:A,a>545:A,A}
jv{s<985:R,R}
ntd{m<1508:R,s<3595:R,x>3253:R,A}
in{a>1848:qv,xc}
tq{x<2961:A,m<1415:A,A}
nnc{a>1547:spz,s<1809:dxp,tkk}
spn{m<1464:fd,qq}
lb{m>1048:jj,s>3042:gj,jbm}
jp{x>352:R,x>150:dp,R}
xdd{m>2063:A,x<2439:A,R}
cvd{m>2821:A,s>3344:A,s>3269:R,A}
pmr{x<1969:A,m<1826:rkn,x>2843:R,A}
nxv{a>587:A,R}
ntj{m>223:A,A}
bc{a<2638:A,x>1087:R,s<3706:R,A}
sz{a<1348:R,m>234:A,A}
bdj{x>1322:R,x<496:A,R}
mds{x<960:R,xjs}
qgn{a<3762:xg,fsf}
qx{x<920:zb,s<1141:thn,m<3148:A,cs}
dt{x>798:A,A}
mlm{x>2476:A,a<1363:R,R}
kl{s<819:zcs,a>3222:qtb,tn}
vxs{a>1296:R,R}
jqr{s>340:rz,ln}
kcl{s<2053:A,a<1787:A,a>1822:A,A}
fts{s<3487:nkk,a>344:xnv,a>204:bk,kj}
tms{x>3126:zc,dcn}
zjr{s<1331:R,s>2033:R,R}
zp{x>937:zf,fp}
fmm{x>3629:A,nfq}
gl{m<1938:R,R}
jnc{s<3381:rdh,m>2333:fdc,A}
tk{s<1865:R,A}
mxp{x<3397:A,A}
km{a<2810:A,A}
mmg{m>2930:R,s>931:R,R}
sp{a>794:R,A}
zd{a>574:R,s<3401:R,R}
frs{s<3740:R,A}
ndl{m<3857:R,s<2419:R,R}
mrs{x<3300:R,a>1249:R,A}
llp{m<1166:R,x<2562:A,a>647:A,R}
ps{x>3914:R,x<3885:A,A}
bqc{a<342:R,x<2868:A,m>486:R,A}
ttm{m<3778:A,A}
vz{x>709:R,zpz}
jmh{a>500:A,a>431:dl,R}
nzb{m>967:R,m<569:R,a>1568:A,A}
ccr{a<259:A,x>2863:R,m>2689:jpj,xt}
gcm{a>845:R,R}
crq{x<405:A,a<3566:A,A}
rx{s<478:A,s<601:A,A}
qjv{s>1012:xs,x<3121:qcd,rd}
bnx{a>362:ph,fsr}
sd{a>354:A,s<728:A,x>3339:A,R}
mrx{x>922:R,R}
fn{m>953:A,x>3799:R,x>3648:A,R}
nf{a>2316:R,A}
mgr{m<1019:R,m>1212:R,s>2800:R,R}
crp{s<2967:R,x>3195:gs,cr}
rcj{a>1347:R,s<3802:A,a<1159:R,R}
ngm{s>2669:R,x<772:A,a<2428:A,R}
mtt{x<2039:A,s>2869:A,R}
dxn{m<3811:jrd,m<3901:ndl,cbt}
kjq{a>431:R,a<264:R,A}
th{s<3196:A,m>1730:R,A}
dxp{s>1163:gpn,x<3127:lx,npj}
jj{m>1573:R,A}
ln{x<3092:R,s>211:A,m>744:A,A}
jg{s>2265:pt,x<3115:bj,a<404:mm,rtm}
lkp{s>2578:crq,m>3260:R,pf}
ghb{m>2683:A,a<1732:R,R}
lv{a>3058:A,A}
sdc{m<2914:A,s<1396:A,A}
nnn{x>1343:A,a>2612:ntj,m>361:R,kbv}
vvl{m<2350:R,A}
ldl{m>2747:hmc,A}
shx{m>2190:A,a<238:R,a>247:A,A}
vfz{a<1202:R,m<3071:A,R}
jhj{m<1476:A,m>1618:R,R}
xl{a>381:A,A}
jb{x<903:dkc,mbf}
fhz{s>3629:shx,m<1374:qht,x<737:xq,R}
dnj{s<3548:A,x<2304:A,R}
thb{x>2100:A,R}
dq{s>3566:R,x>1277:A,A}
lg{a<254:A,a>438:R,A}
cx{a>479:A,a>228:A,R}
ff{a<3628:psn,vp}
kzk{m<2769:lv,A}
hb{m>818:R,s>3355:A,a>3385:A,A}
fj{m>2909:R,A}
mj{s<833:R,x<3433:A,R}
rs{m>2569:fjx,s<527:mxh,s>645:A,A}
mpf{s<3355:cjd,zt}
nfr{a>2267:R,m<2881:A,A}
bg{a>2312:R,s>2903:R,m<3225:nfr,A}
qqq{x>3579:R,s>2886:R,kg}
xpj{x<3214:mlm,a>1412:A,R}
cjd{a<1502:zxf,a<1636:tl,a>1722:nhl,bn}
xc{a>988:vh,x<2472:lk,m>2559:zr,dgj}
zzm{m>2382:sq,m>1176:vz,zln}
glc{s<2942:A,s>3124:R,R}
dr{a>2668:sch,a>2594:bc,qf}
vfs{m<3136:R,R}
fgf{a<3582:A,x<1299:R,R}
dcn{m<932:cpx,x>2630:xkk,xqr}
nqs{m>2140:kpl,s>3232:xzs,dhq}
tbc{a<2671:R,s>1765:gdd,a>2959:A,pcq}
gdd{a<2971:R,x<2510:R,A}
ds{a>3704:R,x<2120:A,R}
cs{a<628:R,A}
kg{m<3692:A,s<2043:A,x>3517:A,R}
rnf{x<3546:A,m<797:R,s<1696:R,R}
bht{s>1764:A,A}
zxm{x<724:A,R}
tkk{m<3066:rpd,s<3242:mhr,x>3211:zv,vpf}
nvr{s<2686:A,m<3164:A,m<3679:A,A}
lp{m<1027:A,R}
hjc{m>2694:R,m>2613:R,A}
nkk{m<1363:bcr,x<1522:nrl,bcx}
fpk{a>3107:pb,a<3003:jnc,m>1703:kzk,mq}
gp{a<3040:gd,m>1378:gq,nl}
tlr{m<1705:A,s>2580:A,A}
mh{s>1962:qhp,s>747:dx,s<258:fsq,glx}
gh{m>1308:A,A}
vmr{a<738:A,m>1457:R,x>3060:A,A}
cg{s<2097:tt,A}
gxk{a>2615:R,a<2225:A,A}
fsf{x<2308:A,A}
tjp{s>1983:A,m>1026:A,a<257:R,R}
mg{x>222:R,a>738:A,s>2494:A,A}
pb{a<3197:A,R}
spz{m>3305:fs,qg}
frp{s>2735:A,tlr}
pdb{a>262:A,A}
ck{a<1352:R,s<3466:A,a>1434:R,R}
vn{a>3657:jk,a>3491:mvq,xh}
sjj{a>2890:R,m>952:A,R}
zjl{a>924:fj,a<892:R,hs}
ssb{a>913:R,A}
ks{x>1331:R,s<3215:R,A}
ctl{a>173:R,a>171:R,a>170:A,R}
sqs{x>1164:hgz,m>1497:tgb,a>1299:kd,cg}
tc{s<1335:A,a<230:R,R}
ccd{m<2942:A,A}
cbt{s<2443:R,R}
smn{m<2763:A,m>2851:R,A}
rg{s>3085:qd,A}
nl{m<474:gv,x<777:bkd,m>786:R,A}
bjf{x<923:A,a>1999:A,m<3855:A,R}
skb{s>1771:R,A}
zk{m<1691:R,m<2471:A,A}
pkm{x>1950:R,R}
zz{m>3119:R,A}
kpl{x<2613:ddj,s>3217:lzh,vtv}
mm{x<3322:xmp,m>1572:jgm,s>1507:nmn,dfr}
qf{s>3481:R,R}
xfq{s<3879:A,m>2015:A,A}
pt{a>650:sb,vg}
nxt{m>2048:R,s<1452:A,R}
btv{x>3462:R,x<3228:A,s>2866:R,A}
lzh{s>3529:tvc,cvd}
zx{s<2070:A,A}
tlb{m>2349:R,x>307:A,s<3670:R,A}
hj{m>1696:R,a>3732:R,m>1101:R,vc}
thn{x>1882:A,A}
kx{m<835:R,s<3232:A,s<3342:R,R}
zvb{m<1126:dd,a<2294:A,A}
sx{x>3854:A,a<280:A,A}
hfh{x<1739:dhb,m>1945:ldl,pnl}
pm{a<3085:R,m>2291:zph,R}
vtv{s>2743:R,A}
dz{m>722:R,A}
rd{m>3008:lg,jjh}
cv{x<480:R,a>2594:R,s<434:R,A}
vbs{s>3550:A,R}
sxj{m<3632:R,x>3007:R,ttm}
slq{x>888:R,x<478:R,m>2879:R,R}
ffg{s<3659:zgb,m<2427:zhd,x<2842:hl,rxn}
tx{x>2142:R,A}
pgt{x>781:A,s>3585:tlb,R}
gs{x>3337:R,m<3768:A,a>231:A,R}
qvd{a>597:A,R}
bkd{m>1023:R,x<394:A,a>3625:R,R}
xfr{s>2569:R,a>1703:A,R}
dx{a>1655:A,a<1577:R,s>1324:szd,gm}
pl{a>3561:ntd,a<3417:A,m>2017:tgj,cm}
gd{x>810:A,x>276:cv,A}
gbb{a>2816:A,A}
jrd{m>3720:R,R}
qgs{a<837:R,hsz}
rp{a>568:A,x>1581:R,A}
hmc{x<2522:R,a>3537:A,R}
dkc{s>2731:jp,a>573:pxg,hnp}
ljj{s>2481:A,s<1873:R,x<3600:A,A}
zcs{x<2016:gp,a<3249:bv,mxg}
zv{s<3509:gn,A}
xtv{x<3281:A,A}
ljc{m>1982:R,m<946:ssb,gh}
tv{x<2942:A,A}
br{a<622:A,R}
pg{x<3339:R,m>1620:mj,qvd}
nx{m>1015:A,x<2816:A,a<1557:A,R}
gv{a<3443:R,a>3749:R,s<506:A,R}
jdj{m>1937:R,s>3575:lj,hdk}
kvl{s>3102:R,m<2548:fb,nvr}
blr{m>2874:A,a>415:R,R}
dfv{s>953:R,x>3313:tcr,s<379:R,R}
xhs{s>3776:R,a>3396:A,s>3635:A,R}
cd{x<3475:qb,x<3659:rnf,s<1491:dz,R}
dl{s<3757:R,x<1260:A,R}
qtb{a>3736:mhd,hfh}
zln{x>756:R,m<404:vtd,R}
xll{m<3156:A,A}
jkv{s>2534:A,R}
rf{x<1344:R,m<1437:R,rp}
kj{m<1696:jps,a>117:tgf,mds}
dhb{a>3529:A,s>1474:A,xlz}
jtc{x>1956:R,x>779:R,s<1881:R,A}
xmp{x<3213:tc,A}
vd{s>2634:hj,x<2918:dg,npp}
rc{a>319:A,A}
dj{s>283:A,a>691:R,R}
bnv{s<3177:R,A}
rkn{a>3866:A,a<3785:R,x>2655:R,A}
pd{s<3507:jx,s>3724:qgn,zdz}
jgm{s<1492:R,A}
rm{x>369:R,A}
mxg{m>1940:qzg,a<3661:jqr,dzf}
tqj{s<656:A,A}
xsp{s<327:A,x<3195:A,x<3697:A,A}
sfz{m<1623:xsp,x<3394:R,a<2634:R,A}
zpz{m>1762:R,R}
nmn{s<1941:mxp,s<2064:tjp,fl}
xkk{s>1580:tq,x<2811:R,A}
nrl{m<3032:gl,A}
bsg{x>2079:tld,xfn}
mmp{s<2741:A,x<3001:R,A}
xz{s>2315:A,x<3754:A,x>3843:A,R}
cl{m>3621:R,m<3435:R,x<948:R,R}
dn{m>2995:css,m<2495:mv,a<2572:nr,lm}
dg{x>2029:kfl,a<3612:R,a<3759:A,R}
sq{s<2905:ngm,s<3082:zz,R}
dfr{a<262:nd,A}
kz{x<3908:A,m>3718:A,A}
tgb{s>2072:A,s>925:sjl,R}
csc{m>561:ml,s<1514:qcx,x>3739:chf,lcv}
zkm{x<2764:A,m>3601:A,m<3513:jv,tqj}
mf{m<3737:A,A}
zc{x>3499:rjr,dsn}
htm{x<2819:R,s>2778:A,R}
gt{m>1460:R,s<3317:A,s>3767:A,A}
pn{x<1829:fx,m<2524:R,lq}
dzc{m<1189:jbk,m<1798:jhj,xd}
ztd{a<1654:A,a>1759:A,R}
rkk{m>3530:xfr,a>1725:nn,a<1588:R,A}
lcf{a<2365:smj,s>3201:mmr,zzm}
qzg{m<3209:mkz,mxq}
ml{a<634:fn,lsq}
xtl{x>528:R,s<1785:sjv,s<1987:A,R}
kgz{x<3539:sd,s<1047:mf,A}
bj{s>802:kr,dh}
nk{m>2557:jkh,zp}
pln{x<3400:dj,m>3080:qz,m<2885:hjc,rlf}
vjv{x>3075:kgz,zkm}
nt{a<756:A,s>3112:R,A}
snq{m>896:R,m<448:R,m<610:A,R}
fs{x<3039:R,bq}
qrx{x<3460:gng,mdv}
npj{a<1338:A,s<466:R,A}
cb{x<3588:R,a>727:R,A}
qdb{x<531:A,a>3431:R,md}
ls{x>2248:ffg,a>2731:vr,m<1702:dr,vzq}
fsq{x>1255:R,x<547:ztd,A}
gj{m<378:A,A}
lq{s<2002:R,x<2605:R,A}
fgg{a>2396:grd,rmc}
sr{x>2519:R,s>1401:A,s<1167:A,A}
dzf{m>888:vj,R}
px{x<893:A,xfq}
ztb{x>1540:mb,vn}
pqq{s>2784:R,R}
zph{x<2788:A,x<3258:R,R}
zn{x>3755:A,m<2040:A,x<3628:A,R}
rjr{a<1498:R,x>3740:zjr,a<1676:A,R}

{x=1345,m=187,a=274,s=930}
{x=20,m=2192,a=60,s=676}
{x=671,m=626,a=100,s=152}
{x=689,m=412,a=421,s=122}
{x=1731,m=580,a=362,s=1350}
{x=36,m=649,a=981,s=38}
{x=1297,m=599,a=229,s=60}
{x=2834,m=639,a=134,s=1326}
{x=865,m=744,a=1262,s=2732}
{x=517,m=2968,a=639,s=589}
{x=104,m=146,a=39,s=1380}
{x=145,m=514,a=2453,s=1025}
{x=687,m=934,a=21,s=1849}
{x=1723,m=175,a=1943,s=74}
{x=472,m=331,a=543,s=833}
{x=377,m=1266,a=950,s=817}
{x=1465,m=50,a=1282,s=2111}
{x=58,m=858,a=43,s=362}
{x=1438,m=869,a=1244,s=376}
{x=503,m=371,a=357,s=243}
{x=162,m=334,a=1245,s=1004}
{x=143,m=3045,a=323,s=667}
{x=975,m=260,a=743,s=1011}
{x=763,m=856,a=136,s=1562}
{x=1368,m=281,a=367,s=2545}
{x=2054,m=1089,a=268,s=235}
{x=394,m=197,a=2163,s=758}
{x=504,m=1589,a=3239,s=145}
{x=1110,m=2861,a=187,s=1176}
{x=1120,m=84,a=540,s=1345}
{x=237,m=149,a=3151,s=1649}
{x=58,m=1469,a=640,s=2228}
{x=3038,m=2961,a=185,s=1547}
{x=2250,m=364,a=2837,s=1858}
{x=48,m=3452,a=564,s=604}
{x=307,m=232,a=83,s=2055}
{x=357,m=2428,a=2981,s=1214}
{x=1521,m=659,a=144,s=232}
{x=147,m=1558,a=534,s=60}
{x=601,m=34,a=1,s=219}
{x=1016,m=1635,a=557,s=477}
{x=1757,m=290,a=974,s=130}
{x=78,m=357,a=517,s=72}
{x=1096,m=1490,a=567,s=921}
{x=1153,m=1019,a=36,s=2595}
{x=192,m=446,a=3033,s=1157}
{x=196,m=184,a=154,s=474}
{x=1136,m=1187,a=3,s=1408}
{x=1343,m=550,a=1416,s=493}
{x=89,m=220,a=271,s=526}
{x=1607,m=84,a=2392,s=150}
{x=1942,m=1071,a=535,s=3498}
{x=694,m=51,a=640,s=1582}
{x=73,m=1392,a=898,s=952}
{x=55,m=70,a=1293,s=123}
{x=141,m=490,a=1525,s=1256}
{x=248,m=1086,a=48,s=1246}
{x=178,m=2369,a=151,s=303}
{x=549,m=64,a=287,s=313}
{x=536,m=1232,a=8,s=326}
{x=512,m=176,a=248,s=1806}
{x=2664,m=1782,a=1252,s=20}
{x=60,m=1568,a=300,s=773}
{x=69,m=826,a=3039,s=1402}
{x=997,m=509,a=1029,s=1297}
{x=1249,m=2777,a=1088,s=664}
{x=363,m=1196,a=653,s=57}
{x=554,m=633,a=84,s=392}
{x=168,m=321,a=1180,s=378}
{x=49,m=369,a=676,s=1873}
{x=754,m=669,a=195,s=1463}
{x=1298,m=2822,a=1145,s=118}
{x=268,m=36,a=20,s=1382}
{x=362,m=1296,a=1773,s=120}
{x=5,m=479,a=1272,s=191}
{x=240,m=91,a=29,s=782}
{x=299,m=2653,a=150,s=1105}
{x=271,m=245,a=609,s=2319}
{x=1516,m=91,a=87,s=248}
{x=2066,m=15,a=1152,s=1224}
{x=2375,m=532,a=3410,s=1319}
{x=195,m=117,a=548,s=332}
{x=1316,m=1907,a=312,s=2860}
{x=498,m=143,a=111,s=14}
{x=791,m=2481,a=1750,s=109}
{x=2501,m=1645,a=186,s=138}
{x=1226,m=1772,a=1323,s=1074}
{x=157,m=872,a=1702,s=11}
{x=110,m=1782,a=1995,s=42}
{x=225,m=402,a=531,s=1823}
{x=1385,m=1778,a=2048,s=1256}
{x=2042,m=1768,a=183,s=1230}
{x=107,m=908,a=133,s=298}
{x=474,m=1387,a=868,s=219}
{x=2040,m=2389,a=634,s=378}
{x=24,m=160,a=318,s=1525}
{x=2511,m=2025,a=29,s=3066}
{x=521,m=237,a=2490,s=494}
{x=81,m=650,a=47,s=300}
{x=913,m=512,a=1035,s=976}
{x=3137,m=1024,a=1945,s=1553}
{x=2624,m=19,a=2883,s=1392}
{x=431,m=2619,a=106,s=1054}
{x=2879,m=1830,a=2721,s=64}
{x=487,m=2084,a=538,s=986}
{x=1975,m=1505,a=2336,s=2371}
{x=1579,m=2715,a=22,s=173}
{x=104,m=420,a=215,s=507}
{x=606,m=1143,a=650,s=601}
{x=321,m=594,a=1683,s=115}
{x=1957,m=94,a=1928,s=801}
{x=83,m=720,a=846,s=14}
{x=3590,m=125,a=944,s=1443}
{x=17,m=893,a=1983,s=362}
{x=1561,m=1,a=942,s=1559}
{x=315,m=150,a=91,s=2445}
{x=2340,m=542,a=112,s=301}
{x=373,m=53,a=2477,s=4}
{x=505,m=609,a=1069,s=479}
{x=1770,m=1150,a=2763,s=141}
{x=2620,m=310,a=1088,s=1231}
{x=2007,m=1227,a=822,s=1506}
{x=1639,m=1517,a=759,s=1587}
{x=734,m=199,a=54,s=264}
{x=201,m=3285,a=1106,s=1002}
{x=998,m=1988,a=342,s=220}
{x=802,m=11,a=502,s=3234}
{x=329,m=2740,a=408,s=1384}
{x=2399,m=379,a=1418,s=1496}
{x=1530,m=367,a=1589,s=592}
{x=1455,m=1064,a=736,s=862}
{x=1591,m=1523,a=638,s=1161}
{x=1312,m=1037,a=1998,s=423}
{x=1147,m=209,a=236,s=493}
{x=258,m=967,a=484,s=1157}
{x=144,m=240,a=495,s=1754}
{x=1414,m=871,a=3527,s=1422}
{x=340,m=922,a=407,s=354}
{x=272,m=21,a=701,s=2289}
{x=245,m=1479,a=1428,s=506}
{x=605,m=826,a=94,s=1711}
{x=617,m=521,a=2074,s=2048}
{x=314,m=180,a=1052,s=341}
{x=66,m=1371,a=2677,s=907}
{x=2043,m=257,a=573,s=235}
{x=3188,m=977,a=286,s=789}
{x=444,m=237,a=1891,s=280}
{x=1048,m=229,a=722,s=785}
{x=135,m=345,a=310,s=131}
{x=1732,m=1182,a=1268,s=144}
{x=505,m=74,a=1092,s=1575}
{x=82,m=12,a=1627,s=1386}
{x=592,m=52,a=159,s=3}
{x=447,m=140,a=234,s=1535}
{x=1328,m=1478,a=3650,s=851}
{x=684,m=525,a=1206,s=525}
{x=176,m=199,a=876,s=96}
{x=532,m=868,a=374,s=188}
{x=1283,m=1776,a=207,s=647}
{x=125,m=3170,a=1234,s=209}
{x=2789,m=2694,a=1770,s=12}
{x=812,m=232,a=890,s=26}
{x=802,m=401,a=149,s=2388}
{x=185,m=317,a=1569,s=389}
{x=2155,m=401,a=612,s=28}
{x=597,m=2059,a=519,s=514}
{x=239,m=544,a=176,s=1913}
{x=151,m=268,a=67,s=2401}
{x=1074,m=1531,a=855,s=593}
{x=224,m=1081,a=2594,s=1737}
{x=1092,m=204,a=84,s=544}
{x=2070,m=344,a=582,s=2839}
{x=366,m=1247,a=5,s=1654}
{x=3215,m=1483,a=100,s=115}
{x=2757,m=25,a=420,s=2919}
{x=185,m=189,a=2308,s=2276}
{x=199,m=504,a=64,s=1123}
{x=41,m=196,a=1056,s=3393}
{x=65,m=3576,a=355,s=353}
{x=240,m=209,a=25,s=516}
{x=3870,m=2209,a=959,s=19}
{x=266,m=699,a=90,s=132}
{x=574,m=69,a=99,s=695}
{x=1351,m=134,a=1152,s=1764}
{x=144,m=2920,a=16,s=848}
{x=1025,m=579,a=106,s=1895}
{x=1303,m=2025,a=2837,s=695}
{x=1863,m=2492,a=105,s=2070}
{x=524,m=2272,a=650,s=1171}
{x=296,m=855,a=2195,s=178}
{x=852,m=883,a=1129,s=1479}
{x=226,m=1069,a=583,s=1460}
{x=1844,m=1967,a=285,s=2029}
{x=237,m=100,a=2208,s=947}
{x=1313,m=159,a=179,s=3167}
{x=2016,m=761,a=648,s=1130}
{x=255,m=1059,a=477,s=1967}
{x=816,m=165,a=279,s=1425}
{x=1955,m=781,a=34,s=4}
{x=122,m=481,a=959,s=1732}";
}
