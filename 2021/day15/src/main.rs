use std::collections::HashMap;

use colour::{green, red};

fn main() {
    println!("++++++++++++");
    println!("PART#1");
    println!("++++++++++++");

    part1(TEST_INPUT);

    println!("-----");

    // part1(INPUT);

    // println!("++++++++++++");
    // println!("PART#2");
    // println!("++++++++++++");

    // part2(TEST_INPUT);

    // println!("-----");

    // part2(INPUT);
}

fn part1(input: &str) {
    let walker: WalkerBestFirst = input.into();
    let lowest_risk_score = walker.get_best_path();
    println!("{}", lowest_risk_score);
}

// struct WalkerNaive {
//     grid: Vec<Vec<u64>>,
//     current_path: Vec<(usize, usize)>,
//     candidate_paths: Vec<Vec<(usize, usize)>>,
// }

const NEIGHBOUR_OFFSETS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

// impl WalkerNaive {
//     pub fn new(grid: Vec<Vec<u64>>) -> WalkerNaive {
//         WalkerNaive {
//             grid,
//             current_path: Vec::new(),
//             candidate_paths: Vec::new(),
//         }
//     }

//     pub fn get_best_path(mut self) -> u64 {
//         self.create_initial_path();
//         let mut best_path = self.current_path.clone();
//         let mut best_score = best_path.iter().map(|(x, y)| self.grid[*y][*x]).sum();
//         'outer: loop {
//             match self.candidate_paths.pop() {
//                 None => break,
//                 Some(cp) => self.current_path = cp,
//             };
//             println!("{} candidates to go after this...", self.candidate_paths.len());
//             // self.print_path(&self.current_path[..]);
//             let (mut x, mut y) = self.current_path[self.current_path.len()-1];
//             let mut current_score: u64 = self.current_path.iter().map(|(x, y)| self.grid[*y][*x]).sum();
//             while x != self.grid_width()-1 || y != self.grid_height()-1 {
//                 match self.classify_neighbours(x, y) {
//                     None => continue 'outer, // give up, got stuck
//                     Some((next_cell, candidate_cells)) => {
//                         x = next_cell.0;
//                         y = next_cell.1;
//                         current_score += self.grid[y][x];
//                         if current_score + (self.grid_width()-x-1 + self.grid_height()-y-1) as u64 >= best_score {
//                             continue 'outer; // give up, can never get lowest score anymore
//                         }
//                         // println!("candidate score {} < best score {}", current_score, best_score);
//                         // keep track of alternatives
//                         for candidate_cell in candidate_cells {
//                             let mut candidate_path = self.current_path.clone();
//                             candidate_path.push(candidate_cell);
//                             self.candidate_paths.push(candidate_path);
//                         }
//                         // continue our journey, still possible to get a best path :)
//                         self.current_path.push(next_cell);
//                     },
//                 }
//             }
//             if current_score < best_score {
//                 best_path = self.current_path.clone();
//                 best_score = current_score;
//             }
//         }
//         self.print_path(&best_path[..]);
//         best_score - self.grid[0][0]  // start cell score is not counted
//     }

//     fn print_path(&self, best_path: &[(usize, usize)]) {
//         for (y, row) in self.grid.iter().enumerate() {
//             for (x, value) in row.iter().enumerate() {
//                 if best_path.iter().any(|(cx, cy)| *cx == x && *cy == y) {
//                     green!("{}", value)
//                 } else {
//                     red!("{}", value)
//                 }
//             }
//             println!("");
//         }
//         println!("");
//         println!("");
//     }

//     fn create_initial_path(&mut self) {
//         // start in left-top, target is bottom-right,
//         // start with a baseline first...

//         let mut x = 0;
//         let mut y = 0;
//         self.current_path.push((x, y));
//         while x != self.grid_width()-1 || y != self.grid_height()-1 {
//             let orig_x = x;
//             let orig_y = y;
//             if self.current_path.len() % 2 == 0 && x < self.grid_width() {
//                 x += 1;
//             } else {
//                 y += 1;
//             }
//             for (offset_x, offset_y) in NEIGHBOUR_OFFSETS {
//                 let cx = ((orig_x as isize)+offset_x) as usize;
//                 let cy = ((orig_y as isize)+offset_y) as usize;
//                 if cx != x && cy != y && self.get_cell_score(cx, cy).is_some() {
//                     let mut candidate_path = self.current_path.clone();
//                     candidate_path.push((cx, cy));
//                     self.candidate_paths.push(candidate_path);
//                 }
//             }
//             self.current_path.push((x, y));
//         }
//     }

//     fn get_cell_score(&self, x: usize, y: usize) -> Option<u64> {
//         if x >= self.grid_width() || y >= self.grid_height() {
//             // invalid cell
//             return None;
//         }
//         if self.current_path.iter().any(|(cx, cy)| *cx == x && *cy == y) {
//             // duplicate cell
//             return None;
//         }
//         Some(self.grid[y][x])
//     }

//     fn classify_neighbours(&self, x: usize, y: usize) -> Option<((usize, usize), Vec<(usize, usize)>)> {
//         let mut candidates = Vec::new();
//         let mut best_candidate = None;
//         for (offset_x, offset_y) in NEIGHBOUR_OFFSETS {
//             let x = ((x as isize)+offset_x) as usize;
//             let y = ((y as isize)+offset_y) as usize;
//             if let Some(score) = self.get_cell_score(x, y) {
//                 match best_candidate {
//                     None => best_candidate = Some((x, y, score)),
//                     Some((cx, cy, cscore)) => if score < cscore {
//                         best_candidate = Some((x, y, score));
//                         candidates.push((cx, cy));
//                     } else {
//                         candidates.push((x, y));
//                     }
//                 }
//             }
//         }
//         best_candidate.and_then(|(x, y, _score)| Some(((x, y), candidates)))
//     }

//     fn grid_width(&self) -> usize {
//         self.grid[0].len()
//     }

//     fn grid_height(&self) -> usize {
//         self.grid.len()
//     }
// }

// impl From<&str> for WalkerNaive {
//     fn from(s: &str) -> WalkerNaive {
//         let mut grid = Vec::new();
//         for line in s.split("\n") {
//             let row: Vec<u64> = line.as_bytes().iter().map(|b| (b - 48) as u64).collect();
//             grid.push(row);
//         }
//         WalkerNaive::new(grid)
//     }
// }

struct WalkerBestFirst {
    grid: Vec<Vec<u64>>,
}

impl WalkerBestFirst {
    pub fn new(grid: Vec<Vec<u64>>) -> WalkerBestFirst {
        WalkerBestFirst { grid }
    }

    pub fn get_best_path(self) -> u64 {
        let mut cached_scores: HashMap<(usize, usize), u64> = HashMap::new();
        cached_scores.insert((0, 0), 0);

        let mut guess_scores: HashMap<(usize, usize), u64> = HashMap::new();
        guess_scores.insert((0, 0), self.ideal_remain_score(0, 0));

        let mut previous_cell_mapping: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

        let mut candidates = vec![(0, 0)];

        while !candidates.is_empty() {
            // take candidate with lowest estimate score
            let (current_idx, _best_score): (usize, u64) = candidates[1..].iter().enumerate().fold(
                (0, guess_scores[&candidates[0]]),
                |(best_idx, best_score), (cur_idx, cur_point)| {
                    let score = guess_scores[&cur_point];
                    if score <= best_score {
                        (cur_idx, score)
                    } else {
                        (best_idx, best_score)
                    }
                },
            );
            let current = candidates.swap_remove(current_idx);

            // check if goal has reached
            if current == (self.grid_width() - 1, self.grid_height() - 1) {
                // reached goal, reconstruct map and return :)
                let mut best_path = vec![current];
                let mut current = current;
                loop {
                    match previous_cell_mapping.get(&current) {
                        None => {
                            self.print_score_cache(&best_path[..], &cached_scores);
                            println!("---");
                            self.print_score_cache(&best_path[..], &guess_scores);
                            println!("---");
                            self.print_path(&best_path[..]);
                            println!("---");
                            return best_path.iter().map(|(x, y)| self.grid[*y][*x]).sum();
                        }
                        Some(previous) => {
                            best_path.push(*previous);
                            current = *previous;
                        }
                    }
                }
            }

            // consider each neighbour
            for (offset_x, offset_y) in NEIGHBOUR_OFFSETS {
                let x = (current.0 as isize) + offset_x;
                if x < 0 || x as usize >= self.grid_width() {
                    continue; // invalid: x = out of range
                }
                let y = (current.1 as isize) + offset_y;
                if y < 0 || y as usize >= self.grid_height() {
                    continue; // invalid: y = out of range
                }

                let (x, y) = (x as usize, y as usize);
                let neighbour_score = cached_scores[&current] + self.grid[y][x];
                let neighbour = (x, y);
                match cached_scores.get(&neighbour) {
                    None => (),
                    Some(previous_score) => {
                        if *previous_score <= neighbour_score {
                            // skip as new score is higher than the one we already known one
                            continue;
                        }
                    }
                };
                previous_cell_mapping.insert(neighbour, current);
                cached_scores.insert(neighbour, neighbour_score);
                guess_scores.insert(neighbour, neighbour_score + self.ideal_remain_score(neighbour.0, neighbour.1));
                if !candidates.iter().any(|candidate| *candidate == neighbour) {
                    candidates.push(neighbour);
                }
            }
        }

        panic!("goal never reached")
    }

    // TODO: print heat map of guess scores,
    // this probably can tell us how well we're doing in our guesses...

    fn ideal_remain_score(&self, x: usize, y: usize) -> u64 {
        let mut sum = 0;
        for y in y..self.grid_height() {
            for x in x..self.grid_width() {
                sum += self.grid[y][x];
            }
        }
        let avg = sum as usize * (self.grid_height() - y) / (self.grid_width() - x);
        ((self.grid_width() - x - 1 + self.grid_height() - y - 1) * avg) as u64
    }

    fn print_path(&self, best_path: &[(usize, usize)]) {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, value) in row.iter().enumerate() {
                if best_path.iter().any(|(cx, cy)| *cx == x && *cy == y) {
                    green!("{}", value)
                } else {
                    red!("{}", value)
                }
            }
            println!("");
        }
        println!("");
        println!("");
    }

    fn print_score_cache(&self, best_path: &[(usize, usize)], scores: &HashMap<(usize, usize), u64>) {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, value) in row.iter().enumerate() {
                if let Some(score) = scores.get(&(x, y)) {
                    if best_path.iter().any(|(cx, cy)| *cx == x && *cy == y) {
                        green!("{}", format!("{: <8}", score));
                    } else {
                        red!("{}", format!("{: <8}", score));
                    }
                } else {
                    print!("{}", format!("{: <8}", "?"));
                }
            }
            println!("");
        }
        println!("");
        println!("");
    }

    fn grid_width(&self) -> usize {
        self.grid[0].len()
    }

    fn grid_height(&self) -> usize {
        self.grid.len()
    }
}

impl From<&str> for WalkerBestFirst {
    fn from(s: &str) -> WalkerBestFirst {
        let mut grid = Vec::new();
        for line in s.split("\n") {
            let row: Vec<u64> = line.as_bytes().iter().map(|b| (b - 48) as u64).collect();
            grid.push(row);
        }
        WalkerBestFirst::new(grid)
    }
}

fn part2(input: &str) {}

const TEST_INPUT: &'static str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

const INPUT: &'static str = "1968464916112191391611281151919845361299153415891198154268959132641114694817612267218838462583491879
1525711216133142195619415736511844811196712782556111378172222212112846932272411896119155816624819212
2143631162361912221922237361391393339328231912118313179363187197511971887961771617331732194239586227
2521625314121441899413948951224935654611261168644252173433111379118144918591261352845133931111363211
3419116395171316754263144619316121119141789791156176133491119172413835731812143128339462157211714832
4524964153247428319314129912911299537321149133376523131712395239297611385551521232187248866732262314
3711569922572112311111231381621569819613115824152623211149153892114351938526483513994131899221246184
9372218211227923291116514163647131281542651458928544319272115333829653212283218213572346151271148314
7252439612436928721197324142761165133345614518725427643227211291261848311822364911669219447123922134
3124222285338327514911622121557639192231775721619241455811912419714186296111754892144116269518316174
5113431512416811186162119586184219513154115654732512834832944622331272212673422211413215473168541114
8918761263115111432573131417344914456815121312151143626751572997964931367144931111322221919431184539
2315112868211215119123391436214851892132438117198411361732935213389219321913111286356991581617112311
8213439278426518916841321158915338871751157511511512411526362572186581353252493251161642321331319238
3814648212312217732224215365116422292413315931251182667691212141251192261129314133611341111121183625
1632153282115827199494219478516848819429439421146162464381572116271251127191323111921221781821561229
2964993982275411123511321114194422113815922449458312261195939424115899991846139318433141623175699422
3185911132222225882128911425231711193121162114993245158813396433493132142156112353212191881311222243
2597419176117872623372257547963288114359198274123111217111515117134582471213835379443482187231716921
1156128452723117148178522621611191136116514111135527613613518738572952213915169837111312119876125721
8232332331311591318439824111216968834143211892318778976161332121429244151178389953221281123441111113
2321966882423191161139314598311457179989165191214751117411811412971757518116913242314292957532412983
5176118365231113228253112189957159351158513473428824359199457932137469329915812169531133176131715211
7651131457228491985123334121791142316224345141412388137921745363819214421812123122111415171218423211
3462391917297997812111394356232391771437391419149441431342312821371511368751471167152824426236119229
4673185152112592819181865591613249119154112282337391342192979211292112114221121795165339253212146534
4234957215119118622851122121288184214119123159675381781924921392139651738861641324135449311387528341
5219623922594391372919236718332186133284187961852865774318269243651818144191524996159111313242918421
7163473192149725221212261161211451212213312989111233514933331991243913291411726828442295192918336141
9122121522342158355114591229418954411929582133466671811425725985313519796543513459188229991411919219
1792321481168175515583211914355119425236113438217255811327548818533439118112225647992411894311221942
6235961521362362331146957118417134195221372115174333562114577921453125915976317129248126331176514522
1315322174651119927411853861313118318521851637512417779171141549468491216918151477196121583981631755
3941512442333131733964938141131332181187172121911135919752637339812991513231411212596255623611113539
4221411783221181622948692351164312639511911592521128119211739941244427471111892925571131192573439919
1179196115212251261629513312991614794161983441112259261137256511915723118319332739142153922123143117
4231711395216212819875423823212711161196111827533224932197941859731111442899614524483228527617386378
3918246317283515329158129197374311321211714111832117967254272122116518731213341117948182114211224847
2923818147359817264926711219234482591493138155147251119914129565392391271176111137476683621547214217
1961381361734232123843911261247113619353513993323262598118181179514371422883549929231394584165211832
8514111722951326162613191813346231172214441618781818362213597235884114841316169219182995612563233821
1365732542831964731216313272619283721242171451815356121331113311623984432353118721938919562191451111
2822939361319198155191312193217341726142263417942332122271866654334531117199731412186215315146877927
1111454359813524123761969515729922437711147235265451492183352723992156214643584182711682754226151263
2977234336552255461792636551916673213921564434121847731352815474153152211858923499436981622823142348
1292116131548485823181118519834219267112994518194931139252194529133187389422531711195274926683516371
4911191361188766113552115172265138293791321889212169645451869671991412161748416412312612856411243186
1317171178186646329441152426228813263661198721323128341111326617211726491932473921125915111382162198
1185112194772921899819164227599216312811921688121293758432648221118311682142211822211438864221158113
4542913914281291351283313249391816267798761311622914884369113218553165256697957132933611942925155261
4494431146129213351534929615418612524813688423431931398914972311152141513154988119419614671251497111
9623781247216311684114814216291279421121111133322113443333261553655145212128395757147123414315523289
5321158321169288211432611617339989115915699522248123124468123741112916981113251279212342226718119572
3474561211372227212241133695812932236239356434494845611611717313598811351571451453513995711211431833
8112861219127115311321991221176277548738413781747479942776125183321215594862116111775124614156181132
4962761786242336114125151132266391316125134395336121111216174625111123821122755643641261588223833941
4198791539415131939214719356144111462292386514121424212196639132151862123635134187812272994532554718
4598231121311615611917161815226991221461112165262118216421252531152312223292442184491316842394264422
6239922321177627293489322115395696297817393991218913471197221121271165841514312479855362119521884969
1129114429442542251149132165119424383113411921471754121121214319237527583191735143212464219732392112
1651231731856554191881141112413451251172218323241241662928111731271512822142161717213123118613613958
4298112947145118999241658624222911212348432972424642451312255991211325333571286324481199944628374392
2761216221111199295216838121731331169992911428481193317316457213225122995618911861614181331811151242
4129378113472816127482433662823529814712133421698191248182391953151395229111932982436827873314521119
1591561822112619623231519742542149141689376414216738233473231111332139316451913219341561438221397874
3927251379311586676539123444146199319136274351291218146428138112816811112992452159548481616112158814
9412114216661113751181221282182423133313612415161191211331589216172472359989471911138112692441934134
2223225313925912191371119962371229122143953359523119746452516412252465926211817191299112619176196217
1346158412389516131916991159321171727892967134931688661131511835391329289921283696234673819265582175
1522461841192567446612342131217417151616649916394676931424334131645279316115831134257161411114339316
2918271212143975123191492589111415816122658247911522114911751889713836923119752121583492316934222783
2335822567917124981467893564192215151353297122123129212261122319967914112586924611514581622672125171
5262413541889891181929911314153212728724291811815629411322228137335534214312958167399318199354579921
9149219511885776119163152492427744911141613211121114715212251229291241137566488625114213223212151148
1181157118724317911213711358719188263574762499564922141213462748611178379228521173951691928154154412
3213211513627293819429164224237122932214895451454614242621317522471165136382297232654725159128311114
4484732912114211553149811921421351313119141219414157143619532522811639911376191311111137331991152913
1262277731165542116111282477417427466617265711322672119177456936322531711241935314283119112931271932
8214487123411419964129118129254247918442832196374131848261379849412854112626271519928536312355117313
1832123949338931145313411211931537496181431511219182279284211859131676523211671295264677294311151228
1113331952811742119875195316449591311261618916825111517632719398311919141292213725149811219913814311
2183432231291181121671961945221637251379192299935371391132618747113147599983122713361339112821364318
1241549642129923318162112113915337861432341899721968116332321232337371311172285444693912311175852346
6943425149126562165738742539622666221429574222528297996511562211917121698495184521192211121392891925
3953911963215814161264113995511685544211797184319295131476823118129122192826662468114521116322441136
6116346345651121664941991121121311413328951552314256113457889116194327118619533491348898152231235194
1953441191249162186952321591361417162859225311123722633854535156121327211121914511731241246272342113
7116541293114612191134544214846918511122329126131849271259313569415571115134141815215312113894888172
2129212414488722182372398711847475194188444222531918316111338511967921142317492122141511831529112878
7299971148111133521137477211291393539983146478512354451435418118599937354197118811351825126562471838
3221753129222942924161143512659426819184812455272916714683133423132121128513161681618521251882367126
7514395613933634141312211145243355553271117322417232972862411211881312311684714718127539314131613781
3755164181884737344157241191441917282435985872268369128914871211246781489161614827272731214553141912
1213151346851191527143987929944819185122171119431928512718341126514385311264471629211116125461472177
7945119111919535234662132621944641673221471944118212216712891297182783944744711859271919395411258142
2833123313293199844711314521199719361191931438661222212228993111111497111395122621712972371139939619
4321293489656698521653282414295113191986252764513991554331562212629412621454111114271569299317726181
3131241189731335231214319872311119789921512392816123597387128412667119915386414217195129492221913482
7136132513132281639712311618231211268291831313425125189328161681993353119711511196639111622963441181
4516311181565123296194779671456321611329615324411382743419111219951224135411334613154142168712942561";
