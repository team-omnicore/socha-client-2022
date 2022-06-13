use crate::game::Piece;

const PIECE_KEYS: [[[[u64; 2]; 2]; 4]; 64] = [
    [
        [
            [14934190605530153734, 1263386223068951700],
            [10428174265610763051, 12495636760962405104],
        ],
        [
            [16527498214959054861, 16360126879818682553],
            [8890115572743122548, 3691314269368829389],
        ],
        [
            [2617666225687429902, 2993981841138757426],
            [868397841763725076, 1276153375879847046],
        ],
        [
            [10558582559018909684, 15808798827889688998],
            [16062304175150365797, 10572205499805811811],
        ],
    ],
    [
        [
            [5423246135365531243, 13003917390177242473],
            [10159369961257934043, 17099326320753791253],
        ],
        [
            [756951344372289784, 16145212906343546289],
            [3871049315859103853, 545903634487997558],
        ],
        [
            [16102839502030717805, 12664378121974401040],
            [11070146900917797488, 3837119521342431581],
        ],
        [
            [3333921901610673899, 12394209326723622305],
            [931123744105632429, 15204928252775034132],
        ],
    ],
    [
        [
            [11129304226710449964, 5996735172983392591],
            [7095515484301377493, 10837556087943730510],
        ],
        [
            [9563136626222007031, 7073793913768039332],
            [4574378210381961193, 16191200688796257326],
        ],
        [
            [7866533537239820650, 1567841549096054061],
            [5799832083814991492, 1131586036534087148],
        ],
        [
            [5893081408479768216, 18322559332087113904],
            [13350367053021864611, 3966135438375543181],
        ],
    ],
    [
        [
            [9651712750059859214, 14984888302131361020],
            [1125148650355961164, 11498466026810561708],
        ],
        [
            [2314100054442072721, 10588700437584065417],
            [8108879697888796244, 2241795864063241031],
        ],
        [
            [11627424035149645426, 10723128116534265613],
            [15220514625005613349, 17474862082815941111],
        ],
        [
            [5525187830230447340, 14549587999355484206],
            [16377438006195789790, 4811442528556148597],
        ],
    ],
    [
        [
            [16943315160430758592, 6587416940541552299],
            [17200076642750833427, 18399585922018572037],
        ],
        [
            [8178322928203422340, 16317157290775320002],
            [7756582167512578444, 7966214267748716230],
        ],
        [
            [188291080989986950, 8718752699873301172],
            [1581804336244378259, 7645177033753947720],
        ],
        [
            [13652763560643064828, 14334700310410215647],
            [10189052946268319037, 629008591006449305],
        ],
    ],
    [
        [
            [6484092854347354237, 12433100235237276132],
            [7628403632579138887, 16588160580368624068],
        ],
        [
            [7943991482164777079, 9671925776135351631],
            [12998339092373529697, 10442025571468422845],
        ],
        [
            [913330596733219948, 1101291195308242150],
            [16432610174929565072, 3747003564994209017],
        ],
        [
            [3218100666794752121, 10962192569246640912],
            [15436044944737671414, 17790784523030056772],
        ],
    ],
    [
        [
            [9477833392288147916, 15457265317106316762],
            [281087723074059133, 8596145278937249835],
        ],
        [
            [15041691745986916634, 3183716627321723064],
            [268807265723726592, 14898875842231885535],
        ],
        [
            [12599518057650445602, 606811771246522453],
            [15550477158813493927, 17586083646058053629],
        ],
        [
            [580770125048548778, 12057610005396393631],
            [9221024745055292660, 14219855125130465847],
        ],
    ],
    [
        [
            [10506216538900446626, 2299322688586827672],
            [10781403241711733114, 1850292664435533321],
        ],
        [
            [14985433481549928422, 4074628175078534067],
            [12276311136535478070, 12854777706396636696],
        ],
        [
            [3255963862534170757, 4039365393463631736],
            [10436860547119032042, 634089610703595814],
        ],
        [
            [4283178055557872838, 6792256830011626465],
            [11145972661542473234, 11720485054785028463],
        ],
    ],
    [
        [
            [1917235479949067899, 5014526881101853921],
            [16555499878654274180, 5747483524067039754],
        ],
        [
            [9696002006834067736, 17389433018655029936],
            [9784311287557706383, 701993017802346550],
        ],
        [
            [5248746229407889990, 11478800147790183262],
            [13460906006311150739, 10027472363406752760],
        ],
        [
            [16031688024795076686, 11863193285893427019],
            [13794777347142759693, 2112966148543764624],
        ],
    ],
    [
        [
            [15469621981555607017, 5023590181896600721],
            [13886917923393816102, 18315135988940987737],
        ],
        [
            [2981344134667139689, 6468957980314773055],
            [3291481613754997125, 10454874520218500395],
        ],
        [
            [164984194946560550, 4932021166845539824],
            [12356347146180179470, 6746893402371932209],
        ],
        [
            [2601352895714138805, 6632760803978746185],
            [7423366873938253109, 12952329352993228622],
        ],
    ],
    [
        [
            [16568415771109654045, 12744197817157771302],
            [2652796172442685812, 6721170332580719336],
        ],
        [
            [7492644327876369704, 6766893477844675206],
            [4386976801478592113, 13228948998754922571],
        ],
        [
            [5943892792353469200, 7917811410373599535],
            [6104490924347752747, 11043302853008129387],
        ],
        [
            [8411283231953121035, 12727150738146927634],
            [17167901362800871909, 2987962964828885335],
        ],
    ],
    [
        [
            [7299895805298545300, 11445662888282745225],
            [3044926994793987851, 16122483616758510338],
        ],
        [
            [1569849377662691909, 158313505719359303],
            [1298916486120994520, 17820581373476910082],
        ],
        [
            [2259735335208180286, 10904689913685299472],
            [9921684553158201406, 6541915828880440067],
        ],
        [
            [7617473840205257021, 14844201437489042677],
            [5366960999252064990, 3114399802740688115],
        ],
    ],
    [
        [
            [16903157153160782305, 3715895889614121430],
            [367980945064044041, 18356760608577138860],
        ],
        [
            [1247990732100912437, 1883104126776890105],
            [7182085697552552844, 16550219880376263898],
        ],
        [
            [14898721519033238231, 16646830692328937578],
            [2362390209858920029, 2304722777754307891],
        ],
        [
            [13869037703830786293, 643104128196559089],
            [18332771770403474486, 12177905463265904224],
        ],
    ],
    [
        [
            [16548018901253486698, 8113631370667515928],
            [10827339021438600443, 2968199031418762908],
        ],
        [
            [6456071706673582209, 13035684911317711533],
            [165024375229809951, 7864296630146810450],
        ],
        [
            [11397749725974561241, 2728296268201750751],
            [17498973099746450812, 757364800649806012],
        ],
        [
            [2279209847245727240, 18316797295310032512],
            [632886094169864918, 1964378118102173209],
        ],
    ],
    [
        [
            [8602540613192290915, 2130119753062140907],
            [6437681024239161896, 2425986362640850226],
        ],
        [
            [4216528755495089519, 13741088270192464238],
            [10258236961454187263, 7650983665026791912],
        ],
        [
            [1805961874159789133, 9904668440929545142],
            [10643024675650775662, 4279262598168145161],
        ],
        [
            [3904493217667360265, 5129159584497412563],
            [3614714055112088801, 8021048257931368394],
        ],
    ],
    [
        [
            [7274544022324087225, 15893396027858321918],
            [15191720450799413709, 17411337103472003732],
        ],
        [
            [10916249619342685962, 12425603013873877951],
            [13017600848121358971, 5215291877953533374],
        ],
        [
            [17593943445591951677, 8628914280792334418],
            [2131961984516571611, 5330915454417498657],
        ],
        [
            [17791632052913120387, 2140483999848665175],
            [14864635221717916239, 907010314363863542],
        ],
    ],
    [
        [
            [3738060476087385550, 12188346456652532223],
            [969322881537488491, 276399586631228475],
        ],
        [
            [15722051022740468028, 3001022761950652615],
            [3078794281578806876, 1497080821760491880],
        ],
        [
            [10239127154188788993, 14736807363276085526],
            [16613237273375642210, 8571695254083536632],
        ],
        [
            [2779869955229447158, 2264667264807951137],
            [16208747945388075151, 2133105038710960998],
        ],
    ],
    [
        [
            [13891093096206942477, 9820317190219899519],
            [6016425680015192127, 2261889626505692617],
        ],
        [
            [9865819057926234344, 12984330818203984416],
            [7075984668684889732, 15095653318551853251],
        ],
        [
            [9316228429984976608, 10009181533452046730],
            [11222373964350816134, 2232204558853515639],
        ],
        [
            [6766284280001362290, 9708624709770163680],
            [8164188111964281890, 3842588403464833068],
        ],
    ],
    [
        [
            [2829317239484409518, 4307806350714859981],
            [16393083168713691149, 7611625655009687462],
        ],
        [
            [3895333061984351771, 4557039790373431794],
            [8340094724157399878, 10109720493140923354],
        ],
        [
            [1729318198559452256, 13621446586684506825],
            [1008586536444094360, 180610691957775851],
        ],
        [
            [1107424608125556155, 6785486636205726715],
            [11569075367654005366, 1793080223354271298],
        ],
    ],
    [
        [
            [15490143656948733974, 12111586906493737353],
            [17131340462389643363, 17530991886596225367],
        ],
        [
            [14681917267683341572, 2913341935374157751],
            [4731052974711283119, 13522468370667395615],
        ],
        [
            [4470899937381239152, 9156050299834604708],
            [13120483087098070181, 5546734150945050918],
        ],
        [
            [3641864614024130894, 10750449703452854724],
            [9008171534778504580, 9500242009750012993],
        ],
    ],
    [
        [
            [1398041464052241248, 11353808727961352664],
            [5524630909854892190, 2949388275619278193],
        ],
        [
            [13231177929506729461, 7721302783418850002],
            [4006286961285672925, 11679866919154267580],
        ],
        [
            [5328293058230474385, 11773434417980876815],
            [1025719599740417052, 15785215256791797489],
        ],
        [
            [8939860206784259672, 14627235783461795077],
            [82526934115402041, 15086705286647848751],
        ],
    ],
    [
        [
            [15166142586613342597, 18228713354643219874],
            [9911194366052608130, 6639500595072587131],
        ],
        [
            [3662051921564668270, 3641228626195956236],
            [329314612070947634, 4664641057836672866],
        ],
        [
            [3253345025986159379, 5105364989823294518],
            [9635193634916824705, 5591765614715783139],
        ],
        [
            [542286469559178257, 6546523538915846742],
            [6996374424092045074, 15908348637069856815],
        ],
    ],
    [
        [
            [8669192360828853439, 9755933059109637043],
            [5039042823901625390, 12166029773371747086],
        ],
        [
            [527750428782471351, 1120183004031702442],
            [8248272052768073365, 8610023256550116876],
        ],
        [
            [9851889084113098614, 11866176463030091868],
            [4582541424406825833, 4024609372418273567],
        ],
        [
            [10810699730326725130, 7987446858049283271],
            [17920634304792817309, 10127110772417799087],
        ],
    ],
    [
        [
            [14290802645844590758, 11271493641733154102],
            [11687813764594026425, 5337807146696040758],
        ],
        [
            [5861648044993396423, 2158433123705968734],
            [12961320948434748849, 13596461294534538024],
        ],
        [
            [15287455054739762025, 10223550388026401377],
            [5368383945187420003, 15754464184585361592],
        ],
        [
            [13352435636732911745, 10689356907507278857],
            [13554635737633002996, 5280440211226369889],
        ],
    ],
    [
        [
            [11021052250305100628, 17844175961018765409],
            [1152595023955140167, 4649089159138453353],
        ],
        [
            [6565320706054958341, 7459058173384217028],
            [8899653695515801056, 15854243180611606526],
        ],
        [
            [1525754502966144214, 10453648883349649128],
            [9860817952062049728, 4255317494429110440],
        ],
        [
            [13107154037528378886, 12601998443866134948],
            [148285319474105925, 4953768117463642018],
        ],
    ],
    [
        [
            [8083451352325720841, 14317822778507757115],
            [1930763183891442923, 8767319251648944415],
        ],
        [
            [841711589832805541, 12945488074920842364],
            [9369889049168847443, 17754928644865852857],
        ],
        [
            [6534005363587750861, 10924843429098892868],
            [13344824549156987996, 10381272518691784586],
        ],
        [
            [6479939499076268895, 2989541718597009459],
            [14784930676963177721, 13578095017738290901],
        ],
    ],
    [
        [
            [7764605541044544578, 17435688060613403473],
            [3065429489293841862, 13851208404509488050],
        ],
        [
            [3146995012913567212, 17024760017443699028],
            [6021961132039636047, 7592045709361971964],
        ],
        [
            [12558988371112554293, 14016288573706134388],
            [9121912545247315375, 701274289013465677],
        ],
        [
            [5164661055848975045, 13209973938691608056],
            [13741110268345751616, 42758957133407058],
        ],
    ],
    [
        [
            [7827800689710638865, 17821843999359623755],
            [1543822292346309820, 2301812593126448361],
        ],
        [
            [17446392312837333787, 12352712541285829628],
            [9213578394620508010, 9446596180134234422],
        ],
        [
            [5492141331282020079, 11968860850828580528],
            [17169191444916391365, 16310919396107043367],
        ],
        [
            [3170553819234755555, 18274080593599188976],
            [4239559423224539847, 10295305690783038782],
        ],
    ],
    [
        [
            [3291395022508313260, 2265906986053207711],
            [2310798722823791072, 15958091300285442605],
        ],
        [
            [1213155986535125007, 9985702479903498445],
            [10856693775947831865, 17443856951643481755],
        ],
        [
            [17801989755058424685, 453289118463894331],
            [16928906331092743915, 17006798690808272404],
        ],
        [
            [4458809671318242233, 216379669973449479],
            [13495183162673432771, 15061878406897027929],
        ],
    ],
    [
        [
            [10205491117403831315, 1012666220145228571],
            [7911727296664411489, 635877448300084422],
        ],
        [
            [5410324468395798427, 10142542522407155127],
            [9993524593785237547, 17587185593314414657],
        ],
        [
            [12773351713599017540, 5416214275397503733],
            [6870577057165398988, 13021173208759905918],
        ],
        [
            [2649265754471864292, 3518945564924887535],
            [12520862261992165357, 27071077311767100],
        ],
    ],
    [
        [
            [13714743835645409605, 3089878056854165750],
            [7738220974833058048, 6875189966373725562],
        ],
        [
            [14655905362765338667, 14123379151058723229],
            [4400626748206075425, 6170902965154770369],
        ],
        [
            [14538399593679382934, 8909549059862781362],
            [10918671638505551063, 14812550396900683821],
        ],
        [
            [5135717378047551619, 2305912133266168863],
            [17784049572348090862, 21949114202097359],
        ],
    ],
    [
        [
            [17137115362666938585, 13302414752570494845],
            [11099119221000267289, 11548001627178702724],
        ],
        [
            [2548900247157576933, 7505383226360118433],
            [15412295798660499433, 10485991849402710268],
        ],
        [
            [7218017287246489912, 13323054708871779048],
            [7088774945968250352, 998276899574897531],
        ],
        [
            [12215364249932263195, 5122683714457317297],
            [5235428033321396881, 11402292295831172506],
        ],
    ],
    [
        [
            [12847335233262880980, 6430223380947030020],
            [8713429652082977646, 16030078365622519654],
        ],
        [
            [12383431308900249051, 15280963294622722891],
            [7544307322742686678, 10388713786333493309],
        ],
        [
            [2045425616116527334, 4788452647625783008],
            [14251587413179809954, 17838025106326301083],
        ],
        [
            [1432932307251619064, 10703433380415533641],
            [911909430132269181, 17532214908845248260],
        ],
    ],
    [
        [
            [9312085346467797390, 13299531111226869833],
            [15752025938293477611, 1520709627946840491],
        ],
        [
            [16914109657012650829, 3267905678011344726],
            [12064165614205679108, 12050072275698208019],
        ],
        [
            [8923486540951168369, 16952292611082765894],
            [10648254796732813201, 11688996287718819085],
        ],
        [
            [9522627564007185116, 3601524914343109580],
            [9646239342148211990, 17213622023420878669],
        ],
    ],
    [
        [
            [8422322985377365096, 8117850557005641946],
            [6760445538990687929, 11660145886756427508],
        ],
        [
            [6575206627729673145, 18343118064029413788],
            [17121676568166524984, 5600097418673800296],
        ],
        [
            [5776761529126216186, 2689321122752235948],
            [2021301122628633692, 900152668058072348],
        ],
        [
            [15215332986292692796, 10306287471099462010],
            [10034481848056920452, 1527182288369167783],
        ],
    ],
    [
        [
            [17263201040495451719, 1418751360374035525],
            [13323764214996742619, 17357563636808505341],
        ],
        [
            [16807186696923029171, 11268790123026557059],
            [5165792665110682557, 12021078576041625202],
        ],
        [
            [9250874544441689852, 4435106766823711227],
            [8787453471650888092, 5167448923638809458],
        ],
        [
            [8224520033822025024, 5270335190538635716],
            [10044056866492237197, 17316021709477924366],
        ],
    ],
    [
        [
            [8891766563729974460, 5734569073546068382],
            [4495991738911306827, 8564520666339321034],
        ],
        [
            [1264191106572638219, 18408704468612592629],
            [425936687070607395, 6801254403099725594],
        ],
        [
            [18296393536694211609, 8478013824751469073],
            [7766870527245496607, 10366912881898942543],
        ],
        [
            [12943122540227010550, 16188219727244547449],
            [10174549670094156731, 16047129709607436488],
        ],
    ],
    [
        [
            [13892390694761882088, 9636599684903386297],
            [10094000645957459746, 18441789012991737868],
        ],
        [
            [13349943050031660560, 14190304259417734456],
            [12239771502654177475, 7878788656368207288],
        ],
        [
            [15653800966914391541, 1096265905836206992],
            [9903190091593198520, 17296511892985748891],
        ],
        [
            [13582996700987500555, 7813541904679713359],
            [11544291416200135301, 13958225627079732690],
        ],
    ],
    [
        [
            [9298387397006194279, 7881168991620314443],
            [2046833419289051787, 9421485120321950188],
        ],
        [
            [6713374716081643534, 9894488919020704627],
            [573834259180730977, 16696088864045778255],
        ],
        [
            [13654362539012572300, 210404638645431855],
            [3329762958432267180, 9600265785838214235],
        ],
        [
            [9848755338206168048, 11697267333780149477],
            [9795216378954100142, 12443313166887254059],
        ],
    ],
    [
        [
            [13243394357535318380, 14291013465089202500],
            [2645224518093730394, 4379326724166630412],
        ],
        [
            [7849184581605101791, 4776085380251279902],
            [8402433557493013464, 5561379127918162706],
        ],
        [
            [6079979911283561570, 14666006955883161832],
            [16006396603698535226, 5896772429460993848],
        ],
        [
            [11465811153219616894, 17398573717668027023],
            [10542371643101862412, 11108460124201670043],
        ],
    ],
    [
        [
            [13397074260611953829, 10805821370931294437],
            [8044858766789455632, 17615888575490777374],
        ],
        [
            [12982877822360394401, 7770281640654777874],
            [14784438657293967554, 15237508349296378665],
        ],
        [
            [14143693312323973364, 5623705394243596550],
            [7586604964776623002, 11426139035933299060],
        ],
        [
            [7088176115375824799, 11062101627061473222],
            [2027641734636980956, 6658406514611740782],
        ],
    ],
    [
        [
            [5930944364261818283, 15153227546014764188],
            [8247284117156258975, 13802109224276961544],
        ],
        [
            [15713779581344763740, 3604455639089675716],
            [13861681561385049463, 2989595052975145007],
        ],
        [
            [12871626309126333230, 8866702369464262062],
            [16084709152503470841, 3525116905814490052],
        ],
        [
            [7344392336446116672, 9901791427177874101],
            [6478496408990182928, 7761242347013342608],
        ],
    ],
    [
        [
            [1851951545747416475, 16613493949588476787],
            [13882202784635853183, 14184327722491824922],
        ],
        [
            [5884264632424763796, 1495789420610609263],
            [6493523083970529955, 4711000666725278958],
        ],
        [
            [10799904328487838051, 2187661317906830061],
            [10491352618612805039, 17284067447450179544],
        ],
        [
            [13501550932207915734, 997544956960041709],
            [4451776096516328238, 16826824376501331991],
        ],
    ],
    [
        [
            [1576863611421521056, 2844418519744778447],
            [433695668288322922, 15803028414282262931],
        ],
        [
            [4094081968516324384, 11720403222765589374],
            [1751092311892515977, 6415192006947623942],
        ],
        [
            [12774056606294291691, 9007870723260729006],
            [10174222658641721312, 6269371296684957884],
        ],
        [
            [12770090627068603955, 12852665922430580990],
            [14254077929777211578, 1966211690417652341],
        ],
    ],
    [
        [
            [16740014512894276829, 934255571393998472],
            [8436154266623525402, 1424062511923945554],
        ],
        [
            [8076575123871965273, 5106804299093724892],
            [5926336468440514568, 16413887758018343556],
        ],
        [
            [10423164150994112386, 1596680273302758429],
            [18102547364830757382, 14411597917182016099],
        ],
        [
            [1800417538238477040, 3169473713398440934],
            [14946156668453543811, 11221274049277183508],
        ],
    ],
    [
        [
            [11408305242513215701, 8637466575807299630],
            [4147816377702329899, 9430548611217663539],
        ],
        [
            [12613235213742706361, 522543958411803964],
            [13073803744705993770, 5368142913242369125],
        ],
        [
            [12430980505403252881, 17257841800683538463],
            [11054892573821646563, 3209231886014218852],
        ],
        [
            [1429543977670108792, 12492797502561500077],
            [7852975268813004829, 8538008536383862732],
        ],
    ],
    [
        [
            [14941381006736386298, 11392251760772876089],
            [13726714093109144631, 10873509530475751451],
        ],
        [
            [5673898694724719667, 14879486968107839105],
            [11619046691255068144, 17249255075292240243],
        ],
        [
            [6073523463794077621, 2219659386488302302],
            [6170996126555421639, 1576168348167545943],
        ],
        [
            [17193003006345401274, 12408557973965248382],
            [7454598820420749634, 3215765505402397339],
        ],
    ],
    [
        [
            [11728490471706826194, 14857377766448743236],
            [4452569628252645848, 12000693930095465043],
        ],
        [
            [6060434304944083348, 12364324026164648560],
            [12924719998701303411, 16034809918108708656],
        ],
        [
            [11491491997258010871, 18224670848225722974],
            [1615145193599589248, 7380048671233324233],
        ],
        [
            [4745015267247858911, 7914894587499887302],
            [12467351899380409751, 7360281871812156127],
        ],
    ],
    [
        [
            [16681616603439737460, 1188859396017371537],
            [14080668295493442939, 12942987710285243792],
        ],
        [
            [16032190064331311475, 11752087916741573708],
            [16661902907245742855, 6391673158379930758],
        ],
        [
            [4190997227528621259, 7231211534287766454],
            [12891693173275419910, 15528625275960880588],
        ],
        [
            [16132350314071357512, 16672742554984247166],
            [5505675781598742461, 6019960537756325101],
        ],
    ],
    [
        [
            [7193016519257752472, 11706277135394939147],
            [3376912849463138396, 12042041395997932018],
        ],
        [
            [17286468776557723876, 9198776223546436873],
            [9754488642698759732, 13279649222701577281],
        ],
        [
            [1178040892430780392, 13257600000857165345],
            [2607758926128000190, 1182343305306962547],
        ],
        [
            [1127231511404656412, 6820547289962297497],
            [2129733871657099764, 2734480671357149085],
        ],
    ],
    [
        [
            [2182889251439611640, 16133333763457323287],
            [7089406742720254878, 8687648350913955629],
        ],
        [
            [2974957005907926374, 17202708098234427516],
            [12699832357667059300, 4787605053537244970],
        ],
        [
            [14229655290978038078, 7111869890049129120],
            [15074148278019251149, 5292079570862808237],
        ],
        [
            [12868521222738311249, 5055547497696669070],
            [7591856064123686238, 13056564224084575161],
        ],
    ],
    [
        [
            [2627275858458342403, 2594552194872062967],
            [12259203309074601052, 3758032178570806451],
        ],
        [
            [9183527072070000426, 11534030369971385490],
            [8725295772776727122, 11443759782106595303],
        ],
        [
            [9459622153142006859, 8480872188386496225],
            [16856888524629614420, 4944947296680239967],
        ],
        [
            [9569969575831866384, 11732673331616228033],
            [16624709806791670170, 17165634887541188135],
        ],
    ],
    [
        [
            [7731709533553674517, 4577025597619405542],
            [4677257191749857290, 8308172489997939456],
        ],
        [
            [8867122457199592936, 18393757696427299078],
            [10142360728550696723, 3459584925780143050],
        ],
        [
            [7832255016881806765, 14178722133035807267],
            [8077667377046658904, 15958739361192369356],
        ],
        [
            [6578887332918311445, 6307921030502337163],
            [8216714198166715504, 15521538343879470412],
        ],
    ],
    [
        [
            [12536898988759858849, 3778168751070080967],
            [11798619089525609945, 6477557237065945997],
        ],
        [
            [16212650718053292962, 11059545639889314045],
            [4899861648121536360, 9493653794975281352],
        ],
        [
            [13477509326618640769, 5724974539362274097],
            [11271691769580672115, 15266645565712820961],
        ],
        [
            [199236840466243354, 16097515308069178868],
            [15744883514026228292, 12085714494737917848],
        ],
    ],
    [
        [
            [14374416520432121214, 13741281533221516895],
            [2350948802243977119, 10146613670937231277],
        ],
        [
            [14795767474117305761, 17771202087371901864],
            [4347368225942081321, 13812851818005480274],
        ],
        [
            [15590341334905564322, 11687258427659487859],
            [2805401562135237016, 11215333987200896946],
        ],
        [
            [14750599003622144509, 8131678299731105024],
            [8681469255856719795, 5720740596495450083],
        ],
    ],
    [
        [
            [755759566435620682, 3694998391652058627],
            [8888897693484082392, 3029911781902933869],
        ],
        [
            [8183352129529914185, 9396072554719429440],
            [11410321542039917857, 1387259902306428146],
        ],
        [
            [4265755376398999291, 16791150380174436657],
            [14944111824425918662, 4426921261413508531],
        ],
        [
            [7622691791869902850, 12197894269216015790],
            [13438867631386358794, 12848114216251569660],
        ],
    ],
    [
        [
            [15794718339908287930, 6924284749093583954],
            [3355347518872336770, 13588387496053288703],
        ],
        [
            [15931911880468870132, 11025373628647108655],
            [5219490596271672225, 10641125749292300534],
        ],
        [
            [5158148473362691440, 8439666911044822934],
            [17801591012797057409, 3656923793997235916],
        ],
        [
            [11135532951233579924, 12579659953738265812],
            [1060861101187367227, 17755786076836985591],
        ],
    ],
    [
        [
            [12731876986561460407, 11166670230730755721],
            [5306294585694483666, 2431778213492005882],
        ],
        [
            [6185590127767141012, 3323509901273034901],
            [15095419364201638575, 141893501436019831],
        ],
        [
            [16621042061461655938, 5479420906665098957],
            [5445901358770857839, 14294706083241859278],
        ],
        [
            [9763420705840976246, 15269865564757508469],
            [14815711110808251369, 17233750519860488727],
        ],
    ],
    [
        [
            [2372842524148877095, 9641234732224612380],
            [9681776672935924643, 10513770003100087514],
        ],
        [
            [1975281623438915579, 1194307899464673407],
            [14051181811341190517, 11268104485071516554],
        ],
        [
            [9393728880077368002, 5766169915694909086],
            [12979031760864173268, 12513880755478672893],
        ],
        [
            [514434748069202489, 4716671463533426735],
            [11711167242926452604, 8904881531444176685],
        ],
    ],
    [
        [
            [7067970497079328908, 13388063014608564936],
            [416268557910459450, 6321919216523083897],
        ],
        [
            [8492730688033146784, 624660889418606353],
            [3183857357527043524, 5804451568159284739],
        ],
        [
            [14546412190553477501, 6339790343251312623],
            [12450261251953484016, 573020303407985216],
        ],
        [
            [10120466262645615094, 7596753332483420454],
            [2699821139705260205, 18322465011931417969],
        ],
    ],
    [
        [
            [2136267034943145848, 12643460100025492787],
            [6567086407853244262, 11252868658793974808],
        ],
        [
            [1625049308182156397, 4456064434344117751],
            [18405906299267961354, 101914958080866121],
        ],
        [
            [8043846908508648641, 17737985215416144526],
            [5896330371537233585, 1916141281858918233],
        ],
        [
            [8905080766719913700, 1528467838543403968],
            [15093844801271287496, 16483458632346081946],
        ],
    ],
    [
        [
            [691316707409805120, 14497588747207713389],
            [17699901896564278903, 10827250537578945980],
        ],
        [
            [7231700175272137304, 12280624571306691984],
            [7528310883443448998, 15027309793084679535],
        ],
        [
            [7873973886229751253, 781439212775392861],
            [12272533570886718299, 16406693963458210364],
        ],
        [
            [14420135380748613672, 11746212286779714947],
            [16842697713126310829, 12652106220275358433],
        ],
    ],
    [
        [
            [13681211139949572071, 12923710642017811937],
            [3494558229733940372, 10415454213997399428],
        ],
        [
            [13308735990222295693, 17128648726035601090],
            [8302054971237310748, 5217510482932726638],
        ],
        [
            [2704460410859762933, 13805004303382206475],
            [1604280592522077289, 6085028889285649847],
        ],
        [
            [15936684577475241885, 12520226316467153422],
            [17438084369819810496, 3017663844668662556],
        ],
    ],
    [
        [
            [4721747167911817864, 14366711529556626447],
            [4991174049704547322, 18190031810577407887],
        ],
        [
            [984299584136580864, 8266683663052223766],
            [673794915813420973, 17324694896407381430],
        ],
        [
            [12654963383768125256, 11384322734938640911],
            [11647602100222754930, 1798119812464887136],
        ],
        [
            [6907125482581441537, 8417864892769258256],
            [16880342313396541183, 10991046502476340503],
        ],
    ],
];
const POINTS_KEYS: [[u64; 2]; 4] = [
    [14504811262012591570, 13367911028012462690],
    [16856115647231202696, 15593669500303479902],
    [15358597394133652434, 15125628605256879423],
    [14058016224502456352, 9021514204860379970],
];

/// Calculates the separate hash for a piece
#[inline]
pub fn hash_for_piece(piece: Piece, pos: u8) -> u64 {
    PIECE_KEYS[pos as usize][piece.piece_type as usize][piece.team as usize][piece.stacked as usize]
}

/// Calculates the separate hash for a score
#[inline]
pub fn hash_for_score(score: [u8; 2]) -> u64 {
    POINTS_KEYS[score[0] as usize][0] ^ POINTS_KEYS[score[1] as usize][1]
}

#[cfg(test)]
mod tests {
    use crate::game::{Fen, Gamestate, IGamestate, Move, PieceType, Team};
    use rand::prelude::SliceRandom;
    use rand::{thread_rng, RngCore};

    #[test]
    fn init_zobrist() {
        let mut piece_lookup: [[[[u64; 2]; 2]; 4]; 64] = [[[[0u64; 2]; 2]; 4]; 64];
        let mut points_lookup: [[u64; 2]; 4] = [[0; 2]; 4];

        let mut rng = thread_rng();

        for i in 0..piece_lookup.len() {
            for j in 0..piece_lookup[i].len() {
                for k in 0..piece_lookup[i][j].len() {
                    for l in 0..piece_lookup[i][j][k].len() {
                        piece_lookup[i][j][k][l] = rng.next_u64();
                    }
                }
            }
        }

        for i in 0..points_lookup.len() {
            for j in 0..points_lookup[i].len() {
                points_lookup[i][j] = rng.next_u64();
            }
        }

        println!(
            "const PIECE_LOOKUP: [[[[u64; 2]; 2];4]; 64] = {:?}",
            piece_lookup
        );
        println!("{:?}", points_lookup);
    }

    #[test]
    fn test_init() {
        let mut g = Gamestate::load_fen("mrshmsrh/8/8/8/8/8/8/HRSMHSRM 0 0/0").unwrap();

        loop {
            while !g.game_over() {
                let moves = g.available_moves_current_player();
                let m = moves.choose(&mut thread_rng()).unwrap();
                g.apply_move(&m);
                let h1 = g.hash;
                let h2 = g.recalculate_hash();
                assert_eq!(h1, h2);
            }
            println!("Sucessfully one game");
            g = Gamestate::new_random(&mut thread_rng());
        }
    }
}
