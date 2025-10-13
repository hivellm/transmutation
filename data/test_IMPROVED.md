P rovided proper attribution is provided G oogle hereby grants perm ission to

reproduce the tables and figures in this paper solely for use in journalistic or

scholarly w orks

A tte n tio n I s A ll Y o u N e e d

3

2

0

2

g

u

∗ ∗ ∗ ∗A shish Vasw ani N oam Shazeer N ikiParm ar Jakob U szkoreitA

G oogle B rain G oogle B rain G oogle R esearch G oogle R esearch

2 avaswani google com noam google com nikip google com usz google com

∗ ∗† ∗L lion Jones A idan N G om ez Łukasz K aiser

L G oogle R esearch U niversity ofToronto G oogle B rain

C lukaszkaiser google comllion google com aidan cs toronto edu

s

∗‡Illia Polosukhinc

illia polosukhin gmail com

7

v A b stract

2

6

The dom inantsequence transduction m odels are based on com plex recurrentor7

convolutionalneuralnetw orks thatinclude an encoderand a decoder The best3

perform ing m odels also connectthe encoder and decoder through an attention0

m echanism W e propose a new sim ple netw ork architecture the Transform er

6 based solely on attention m echanism s dispensing w ith recurrence and convolutions

0 entirely Experim ents on tw o m achine translation tasks show these m odels to

7 be superiorin quality w hile being m ore parallelizable and requiring significantly

1 less tim e to train O urm odelachieves 28 4 B LEU on the W M T 2014 English

to G erm an translation task im proving over the existing bestresults includingv

ensem bles by over2 B LEU O n the W M T 2014 English to French translation taski

ourm odelestablishesa new single m odelstate of the artB LEU score of41 8 afterX

training for3 5 days on eightG PU s a sm allfraction ofthe training costs ofther

bestm odelsfrom the literature W e show thatthe Transform ergeneralizesw elltoa

othertasksby applying itsuccessfully to English constituency parsing both w ith

large and lim ited training data

∗Equalcontribution Listing orderisrandom Jakob proposed replacing RNNswith selfattention and started

theeffortto evaluatethisidea A shish w ith Illia designed and im plem ented thefirstTransform erm odelsand

hasbeencruciallyinvolvedineveryaspectofthiswork Noam proposedscaleddotproductattention m ultihead

attention and theparam eterfreeposition representation and becam etheotherperson involved in nearly every

detail Nikidesigned im plem ented tuned and evaluated countlessm odelvariantsin ouroriginalcodebaseand

tensor2tensor Llion also experim ented with novelm odelvariants wasresponsibleforourinitialcodebase and

efficientinferenceandvisualizations LukaszandAidanspentcountlesslongdaysdesigningvariouspartsofand

im plem entingtensor2tensor replacingourearliercodebase greatlyim provingresultsandm assivelyaccelerating

ourresearch

†W ork perform ed whileatGoogleBrain

‡W ork perform ed whileatGoogleResearch

31stConferenceon NeuralInform ation Processing System s NIPS 2017 Long Beach CA USA

1 In trod u ction

R ecurrentneuralnetw orks long shortterm m em ory 13 and gated recurrent 7 neuralnetw orks

in particular have been firm ly established asstate ofthe artapproachesin sequence m odeling and

transduction problem s such as language m odeling and m achine translation 35 2 5 N um erous

effortshavesincecontinued to push theboundariesofrecurrentlanguagem odelsand encoderdecoder

architectures 38 24 15

R ecurrentm odelstypically factorcom putation along the sym bolpositionsofthe inputand output

sequences A ligning the positionsto stepsin com putation tim e they generate a sequence ofhidden

h h tstates asa function ofthe previoushidden state and the inputforposition Thisinherentlyt t 1

sequentialnature precludesparallelization w ithin training exam ples w hich becom escriticalatlonger

sequence lengths asm em ory constraintslim itbatching acrossexam ples R ecentw ork hasachieved

significantim provem entsin com putationalefficiency through factorization tricks 21 and conditional

com putation 32 w hile also im proving m odelperform ance in case ofthe latter The fundam ental

constraintofsequentialcom putation how ever rem ains

A ttention m echanism shave becom e an integralpartofcom pelling sequence m odeling and transduc

tion m odelsin varioustasks allow ing m odeling ofdependenciesw ithoutregard to theirdistance in

the inputoroutputsequences 2 19 In allbuta few cases 27 how ever such attention m echanism s

are used in conjunction w ith a recurrentnetw ork

In this w ork w e propose the Transform er a m odelarchitecture eschew ing recurrence and instead

relying entirely on an attention m echanism to draw globaldependenciesbetw een inputand output

The Transform erallow sforsignificantly m ore parallelization and can reach a new state ofthe artin

translation quality afterbeing trained foraslittle astw elve hourson eightP100 G PU s

2 B ack grou n d

The goalofreducing sequentialcom putation also form sthe foundation ofthe Extended N euralG PU

16 B yteN et 18 and C onvS2S 9 allofw hich use convolutionalneuralnetw orksasbasic building

block com puting hidden representationsin parallelforallinputand outputpositions In these m odels

the num berofoperationsrequired to relate signalsfrom tw o arbitrary inputoroutputpositionsgrow s

in the distance betw een positions linearly forC onvS2S and logarithm ically forB yteN et Thism akes

itm ore difficultto learn dependencies betw een distantpositions 12 In the Transform erthis is

reduced to a constantnum berofoperations albeitatthe costofreduced effective resolution due

to averaging attention w eighted positions an effectw e counteractw ith M ultiH ead A ttention as

described in section 3 2

Self attention som etim escalled intra attention isan attention m echanism relating differentpositions

ofa single sequence in orderto com pute a representation ofthe sequence Self attention hasbeen

used successfully in a variety oftasksincluding reading com prehension abstractive sum m arization

textualentailm entand learning task independentsentence representations 4 27 28 22

End to end m em ory netw orks are based on a recurrentattention m echanism instead ofsequence

aligned recurrence and have been show n to perform w ellon sim ple language question answ ering and

language m odeling tasks 34

To the bestof our know ledge how ever the Transform er is the firsttransduction m odelrelying

entirely on self attention to com pute representationsofitsinputand outputw ithoutusing sequence

aligned R N N sorconvolution In the follow ing sections w e w illdescribe the Transform er m otivate

self attention and discussitsadvantagesoverm odelssuch as 17 18 and 9

3 M od elA rch itectu re

M ostcom petitive neuralsequence transduction m odelshave an encoderdecoderstructure 5 2 35

x xH ere the encoder m aps an inputsequence of sym bolrepresentations to a sequence1 n

z zz zof continuous representations G iven the decoder then generates an output1 n

y ysequence ofsym bols one elem entata tim e A teach step the m odelis auto regressive1 m

10 consum ing the previously generated sym bolsasadditionalinputw hen generating the next

2

Figure 1 The Transform er m odelarchitecture

The Transform erfollow sthisoverallarchitecture using stacked self attention and pointw ise fully

connected layers forboth the encoderand decoder show n in the leftand righthalves ofFigure 1

respectively

3 1 E ncoder and D ecoder Stacks

N 6The encoder is com posed of a stack of identicallayers Each layer has tw oE ncoder

sub layers The firstisa m ultihead self attention m echanism and the second isa sim ple position

w ise fully connected feed forw ard netw ork W e em ploy a residualconnection 11 around each of

the tw o sub layers follow ed by layer norm alization 1 Thatis the outputof each sub layer is

LayerN orm x Sublayer x Sublayer xw here is the function im plem ented by the sub layer

itself To facilitate these residualconnections allsub layersin the m odel asw ellasthe em bedding

d 512layers produce outputsofdim ension model

N 6The decoderisalso com posed ofa stack of identicallayers In addition to the tw oD ecoder

sub layersin each encoderlayer the decoderinsertsa third sub layer w hich perform sm ultihead

attention overthe outputofthe encoderstack Sim ilarto the encoder w e em ploy residualconnections

around each ofthe sub layers follow ed by layernorm alization W e also m odify the self attention

sub layerin the decoderstack to preventpositions from attending to subsequentpositions This

m asking com bined w ith factthatthe outputem beddingsare offsetby one position ensuresthatthe

i ipredictionsforposition can depend only on the know n outputsatpositionslessthan

3 2 A ttention

A n attention function can be described asm apping a query and a setofkey value pairsto an output

w here the query keys values and outputare allvectors The outputiscom puted asa w eighted sum

3

Scaled D otProductA ttention M ultiH ead A ttention

Figure 2 left Scaled D otProductA ttention right M ultiH ead A ttention consists of several

attention layersrunning in parallel

ofthe values w here the w eightassigned to each value iscom puted by a com patibility function ofthe

query w ith the corresponding key

3 2 1 Scaled D ot ProductA ttention

W e callourparticularattention Scaled D otProductA ttention Figure 2 The inputconsists of

d dqueriesand keysofdim ension and valuesofdim ension W e com pute the dotproductsofthek v√

dquery w ith allkeys divide each by and apply a softm ax function to obtain the w eightson thek

values

In practice w e com pute the attention function on a setofqueriessim ultaneously packed together

Q K Vinto a m atrix The keysand valuesare also packed togetherinto m atrices and W e com pute

the m atrix ofoutputsas

TQ K

√A ttention Q K V softm ax V 1

dk

The tw o m ostcom m only used attention functionsare additive attention 2 and dotproduct m ulti

plicative attention D otproductattention isidenticalto ouralgorithm exceptforthe scaling factor

1√of A dditive attention com putesthe com patibility function using a feed forw ard netw ork w ith

dk

a single hidden layer W hile the tw o are sim ilarin theoreticalcom plexity dotproductattention is

m uch fasterand m ore space efficientin practice since itcan be im plem ented using highly optim ized

m atrix m ultiplication code

dW hile forsm allvaluesof the tw o m echanism sperform sim ilarly additive attention outperform sk

ddotproductattention w ithoutscaling forlargervaluesof 3 W e suspectthatforlarge valuesofk

d the dotproductsgrow large in m agnitude pushing the softm ax function into regionsw here ithask

14 √extrem ely sm allgradients To counteractthiseffect w e scale the dotproductsby

dk

3 2 2 M ultiH ead A ttention

dInstead ofperform ing a single attention function w ith dim ensionalkeys valuesand queriesmodel

hw e found itbeneficialto linearly projectthe queries keysand values tim esw ith different learned

d d dlinearprojectionsto and dim ensions respectively O n each ofthese projected versionsofk k v

dqueries keysand valuesw e then perform the attention function in parallel yielding dim ensionalv

4 q kTo illustratewhy thedotproductsgetlarge assum ethatthecom ponentsof and areindependentrandom

dk∑·0 1 q k q k 0 dvariableswith m ean and variance Then theirdotproduct hasm ean and variance

i i ki 1

4

outputvalues These are concatenated and once again projected resulting in the finalvalues as

depicted in Figure 2

M ultihead attention allow sthe m odelto jointly attend to inform ation from differentrepresentation

subspacesatdifferentpositions W ith a single attention head averaging inhibitsthis

OM ultiH ead Q K V C oncat head head W

1 h

Q K V

head A ttention Q W K W V Ww here i i ii

Q × × ×d d K d d V d dvk kmodel model modelR R R

∈ ∈ ∈W W WW here the projectionsare param eterm atrices i ii

×O hd dv modelR∈Wand

h 8In this w ork w e em ploy parallel attention layers or heads For each of these w e use

d d d h 64 D ue to the reduced dim ension ofeach head the totalcom putationalcostk v model

issim ilarto thatofsingle head attention w ith fulldim ensionality

3 2 3 A pplicationsofA ttention in our M odel

The Transform erusesm ultihead attention in three differentw ays

•In encoder decoderattention layers the queries com e from the previous decoderlayer

and the m em ory keys and values com e from the outputofthe encoder This allow s every

position in the decoderto attend overallpositionsin the inputsequence Thism im icsthe

typicalencoder decoder attention m echanism s in sequence to sequence m odels such as

38 2 9

•The encodercontainsself attention layers In a self attention layerallofthe keys values

and queriescom e from the sam e place in thiscase the outputofthe previouslayerin the

encoder Each position in the encodercan attend to allpositionsin the previouslayerofthe

encoder

•Sim ilarly self attention layersin the decoderallow each position in the decoderto attend to

allpositionsin the decoderup to and including thatposition W e need to preventleftw ard

inform ation flow in the decoderto preserve the auto regressive property W e im plem entthis

∞inside ofscaled dotproductattention by m asking out setting to allvaluesin the input

ofthe softm ax w hich correspond to illegalconnections See Figure 2

3 3 Position w ise Feed Forw ard N etw orks

In addition to attention sub layers each ofthe layers in ourencoderand decodercontains a fully

connected feed forw ard netw ork w hich isapplied to each position separately and identically This

consistsoftw o lineartransform ationsw ith a R eLU activation in betw een

F F N x m ax 0 xW b W b 21 1 2 2

W hile the lineartransform ationsare the sam e acrossdifferentpositions they use differentparam eters

from layer to layer A nother w ay of describing this is as tw o convolutions w ith kernel size 1

d 512The dim ensionality of inputand outputis and the inner layer has dim ensionalitymodel

d 2048ff

3 4 E m beddingsand Softm ax

Sim ilarly to othersequence transduction m odels w e use learned em beddings to convertthe input

dtokensand outputtokensto vectorsofdim ension W e also use the usuallearned lineartransformodel

m ation and softm ax function to convertthe decoderoutputto predicted nexttoken probabilities In

ourm odel w e share the sam e w eightm atrix betw een the tw o em bedding layersand the pre softm ax√

dlineartransform ation sim ilarto 30 In the em bedding layers w e m ultiply those w eightsby model

5

Table 1 M axim um path lengths perlayercom plexity and m inim um num berofsequentialoperations

n d kfordifferentlayertypes isthe sequence length isthe representation dim ension isthe kernel

rsize ofconvolutionsand the size ofthe neighborhood in restricted self attention

LayerType C om plexity perLayer Sequential M axim um Path Length

O perations

2 ·O n d O 1 O 1Self A ttention

2·O n d O n O nR ecurrent

2· ·O k n d O 1 O log nC onvolutional

k

· ·O r n d O 1 O n rSelf A ttention restricted

3 5 PositionalE ncoding

Since ourm odelcontainsno recurrence and no convolution in orderforthe m odelto m ake use ofthe

orderofthe sequence w e m ustinjectsom e inform ation aboutthe relative orabsolute position ofthe

tokensin the sequence To thisend w e add positionalencodings to the inputem beddingsatthe

dbottom softhe encoderand decoderstacks The positionalencodingshave the sam e dim ension model

asthe em beddings so thatthe tw o can be sum m ed There are m any choicesofpositionalencodings

learned and fixed 9

In thisw ork w e use sine and cosine functionsofdifferentfrequencies

2i dmodelP E sin pos 10000

pos2i

2i dmodelP E cos pos 10000

pos2i 1

pos iw here isthe position and isthe dim ension Thatis each dim ension ofthe positionalencoding

·2π 10000 2πcorrespondsto a sinusoid The w avelengthsform a geom etric progression from to W e

chose this function because w e hypothesized itw ould allow the m odelto easily learn to attend by

k P Erelative positions since forany fixed offset can be represented as a linearfunction ofpos k

P E pos

W e also experim ented w ith using learned positionalem beddings 9 instead and found thatthe tw o

versions produced nearly identicalresults see Table 3 row E W e chose the sinusoidalversion

because itm ay allow the m odelto extrapolate to sequence lengthslongerthan the onesencountered

during training

4 W hy S elf A tten tion

In this section w e com pare various aspects of self attention layers to the recurrentand convolu

tionallayerscom m only used form apping one variable length sequence ofsym bolrepresentations

dR∈x x z z x zto another sequence of equallength w ith such as a hidden

1 n 1 n i i

layerin a typicalsequence transduction encoderordecoder M otivating ouruse ofself attention w e

considerthree desiderata

O ne isthe totalcom putationalcom plexity perlayer A notheristhe am ountofcom putation thatcan

be parallelized asm easured by the m inim um num berofsequentialoperationsrequired

The third isthe path length betw een long range dependenciesin the netw ork Learning long range

dependenciesisa key challenge in m any sequence transduction tasks O ne key factoraffecting the

ability to learn such dependenciesisthe length ofthe pathsforw ard and backw ard signalshave to

traverse in the netw ork The shorterthese pathsbetw een any com bination ofpositionsin the input

and outputsequences the easieritisto learn long range dependencies 12 H ence w e also com pare

the m axim um path length betw een any tw o inputand outputpositionsin netw orkscom posed ofthe

differentlayertypes

A snoted in Table1 aself attention layerconnectsallpositionsw ith aconstantnum berofsequentially

O nexecuted operations w hereas a recurrentlayer requires sequentialoperations In term s of

com putationalcom plexity self attention layersare fasterthan recurrentlayersw hen the sequence

6

n dlength is sm aller than the representation dim ensionality w hich is m ostoften the case w ith

sentence representationsused by state of the artm odelsin m achine translations such asw ord piece

38 and byte pair 31 representations To im prove com putationalperform ance fortasksinvolving

rvery long sequences self attention could be restricted to considering only a neighborhood ofsize in

the inputsequence centered around the respective outputposition Thisw ould increase the m axim um

O n rpath length to W e plan to investigate thisapproach furtherin future w ork

k nA single convolutionallayerw ith kernelw idth doesnotconnectallpairsofinputand output

O n kpositions D oing so requiresa stack of convolutionallayersin the case ofcontiguouskernels

O log nor in the case ofdilated convolutions 18 increasing the length ofthe longestpathsk

betw een any tw o positionsin the netw ork C onvolutionallayersare generally m ore expensive than

krecurrentlayers by a factorof Separable convolutions 6 how ever decrease the com plexity

2· · ·O k n d n d k nconsiderably to Even w ith how ever the com plexity ofa separable

convolution isequalto the com bination ofa self attention layerand a pointw ise feed forw ard layer

the approach w e take in ourm odel

A ssidebenefit self attention could yield m oreinterpretablem odels W einspectattention distributions

from ourm odelsand presentand discussexam plesin the appendix N otonly do individualattention

headsclearly learn to perform differenttasks m any appearto exhibitbehaviorrelated to the syntactic

and sem antic structure ofthe sentences

5 T rain in g

Thissection describesthe training regim e forourm odels

5 1 Training D ata and B atching

W e trained on the standard W M T 2014 English G erm an datasetconsisting of about4 5 m illion

sentence pairs Sentences w ere encoded using byte pairencoding 3 w hich has a shared source

targetvocabulary ofabout37000 tokens ForEnglish French w e used the significantly largerW M T

2014 English French datasetconsisting of36M sentencesand splittokensinto a 32000 w ord piece

vocabulary 38 Sentence pairsw ere batched togetherby approxim ate sequence length Each training

batch contained a setofsentence pairs containing approxim ately 25000 source tokens and 25000

targettokens

5 2 H ardw are and Schedule

W e trained ourm odels on one m achine w ith 8 N V ID IA P100 G PU s Forourbase m odels using

the hyperparam etersdescribed throughoutthe paper each training step took about0 4 seconds W e

trained the base m odelsfora totalof100 000 stepsor12 hours Forourbig m odels described on the

bottom line oftable 3 step tim e w as1 0 seconds The big m odelsw ere trained for300 000 steps

3 5 days

5 3 O ptim izer

9β 0 9 β 0 98 ϵ 10W e used the A dam optim izer 20 w ith and W e varied the learning

1 2

rate overthe course oftraining according to the form ula

05 05 15· ·lrate d m in step num step num w arm up steps 3

model

w arm up stepsThiscorrespondsto increasing the learning rate linearly forthe first training steps

and decreasing itthereafterproportionally to the inverse square rootofthe step num ber W e used

w arm up steps 4000

5 4 R egularization

W e em ploy three typesofregularization during training

7

Table 2 The Transform erachievesbetterB LEU scoresthan previousstate of the artm odelson the

English to G erm an and English to French new stest2014 testsata fraction ofthe training cost

B LEU Training C ost FLO Ps

M odel

EN D E EN FR EN D E EN FR

B yteN et 18 23 75

20·1 0 10D eep A tt PosU nk 39 39 2

19 20· ·2 3 10 1 4 10G N M T R L 38 24 6 39 92

18 20· ·9 6 10 1 5 10C onvS2S 9 25 16 40 46

19 20· ·2 0 10 1 2 10M oE 32 26 03 40 56

20·8 0 10D eep A tt PosU nk Ensem ble 39 40 4

20 21· ·1 8 10 1 1 10G N M T R L Ensem ble 38 26 30 41 16

19 21· ·7 7 10 1 2 10C onvS2S Ensem ble 9 26 36 41 29

18·3 3 10Transform er base m odel 27 3 38 1

19·2 3 10Transform er big 28 4 41 8

W e apply dropout 33 to the outputofeach sub layer before itisadded to theR esidualD ropout

sub layerinputand norm alized In addition w e apply dropoutto the sum softhe em beddingsand the

positionalencodingsin both the encoderand decoderstacks Forthe base m odel w e use a rate of

P 0 1drop

ϵ 0 1D uring training w e em ployed labelsm oothing ofvalue 36 ThisL abelSm oothing ls

hurtsperplexity asthe m odellearnsto be m ore unsure butim provesaccuracy and B LEU score

6 R esu lts

6 1 M achine Translation

O n the W M T 2014 English to G erm an translation task the big transform erm odel Transform er big

2 0in Table 2 outperform sthe bestpreviously reported m odels including ensem bles by m ore than

28 4B LEU establishing a new state of the artB LEU score of The configuration ofthism odelis

3 5 8listed in the bottom line ofTable 3 Training took dayson P100 G PU s Even ourbase m odel

surpassesallpreviously published m odelsand ensem bles ata fraction ofthe training costofany of

the com petitive m odels

41 0O n the W M T 2014 English to French translation task ourbig m odelachievesa B LEU score of

1 4outperform ing allofthe previously published single m odels atlessthan the training costofthe

previous state of the artm odel The Transform er big m odeltrained forEnglish to French used

P 0 1 0 3dropoutrate instead ofdrop

Forthe base m odels w e used a single m odelobtained by averaging the last5 checkpoints w hich

w ere w ritten at10 m inute intervals Forthe big m odels w e averaged the last20 checkpoints W e

4 α 0 6used beam search w ith a beam size of and length penalty 38 These hyperparam eters

w ere chosen afterexperim entation on the developm entset W e setthe m axim um outputlength during

50inference to inputlength butterm inate early w hen possible 38

Table 2 sum m arizesourresultsand com paresourtranslation quality and training coststo otherm odel

architecturesfrom the literature W e estim ate the num beroffloating pointoperationsused to train a

m odelby m ultiplying the training tim e the num berofG PU sused and an estim ate ofthe sustained

5single precision floating pointcapacity ofeach G PU

6 2 M odelVariations

To evaluate the im portance ofdifferentcom ponentsofthe Transform er w e varied ourbase m odel

in differentw ays m easuring the change in perform ance on English to G erm an translation on the

5W eused valuesof2 8 3 7 6 0 and 9 5 TFLOPS forK80 K40 M 40 and P100 respectively

8

Table 3 Variationson the Transform erarchitecture U nlisted valuesare identicalto those ofthe base

m odel A llm etricsare on the English to G erm an translation developm entset new stest2013 Listed

perplexitiesare perw ordpiece according to ourbyte pairencoding and should notbe com pared to

perw ord perplexities

train PPL B LEU param s

N d d h d d P ϵk v drop lsmodel ff 6

×10steps dev dev

base 6 512 2048 8 64 64 0 1 0 1 100K 4 92 25 8 65

1 512 512 5 29 24 9

4 128 128 5 00 25 5

A

16 32 32 4 91 25 8

32 16 16 5 01 25 4

16 5 16 25 1 58

B

32 5 01 25 4 60

2 6 11 23 7 36

4 5 19 25 3 50

8 4 88 25 5 80

C 256 32 32 5 75 24 5 28

1024 128 128 4 66 26 0 168

1024 5 12 25 4 53

4096 4 75 26 2 90

0 0 5 77 24 6

0 2 4 95 25 5

D

0 0 4 67 25 3

0 2 5 47 25 7

E positionalem bedding instead ofsinusoids 4 92 25 7

big 6 1024 4096 16 0 3 300K 2134 33 26 4

developm entset new stest2013 W e used beam search asdescribed in the previoussection butno

checkpointaveraging W e presentthese resultsin Table 3

In Table3 row s A w evary thenum berofattention headsand theattention key and valuedim ensions

keeping the am ountof com putation constant as described in Section 3 2 2 W hile single head

attention is0 9 B LEU w orse than the bestsetting quality also dropsoffw ith too m any heads

dIn Table 3 row s B w e observe thatreducing the attention key size hurts m odelquality Thisk

suggests thatdeterm ining com patibility is noteasy and thata m ore sophisticated com patibility

function than dotproductm ay be beneficial W e furtherobserve in row s C and D that asexpected

biggerm odelsarebetter and dropoutisvery helpfulin avoiding overfitting In row E w ereplaceour

sinusoidalpositionalencoding w ith learned positionalem beddings 9 and observe nearly identical

resultsto the base m odel

6 3 E nglish C onstituency Parsing

To evaluate ifthe Transform ercan generalize to othertasksw e perform ed experim entson English

constituency parsing Thistask presentsspecific challenges the outputissubjectto strong structural

constraints and is significantly longer than the input Furtherm ore R N N sequence to sequence

m odelshave notbeen able to attain state of the artresultsin sm alldata regim es 37

d 1024W e trained a 4 layertransform erw ith on the W allStreetJournal W SJ portion ofthem odel

Penn Treebank 25 about40K training sentences W e also trained itin a sem isupervised setting

using the largerhigh confidence and B erkleyParsercorpora from w ith approxim ately 17M sentences

37 W e used a vocabulary of16K tokensforthe W SJonly setting and a vocabulary of32K tokens

forthe sem isupervised setting

W e perform ed only a sm allnum berofexperim entsto selectthe dropout both attention and residual

section 5 4 learning ratesand beam size on the Section 22 developm entset allotherparam eters

rem ained unchanged from the English to G erm an base translation m odel D uring inference w e

9

Table 4 The Transform ergeneralizesw ellto English constituency parsing R esultsare on Section 23

ofW SJ

Parser Training W SJ 23 F1

V inyals K aiserelal 2014 37 W SJonly discrim inative 88 3

Petrov etal 2006 29 W SJonly discrim inative 90 4

Zhu etal 2013 40 W SJonly discrim inative 90 4

D yeretal 2016 8 W SJonly discrim inative 91 7

Transform er 4 layers W SJonly discrim inative 91 3

Zhu etal 2013 40 sem isupervised 91 3

H uang H arper 2009 14 sem isupervised 91 3

M cC losky etal 2006 26 sem isupervised 92 1

V inyals K aiserelal 2014 37 sem isupervised 92 1

Transform er 4 layers sem isupervised 92 7

Luong etal 2015 23 m ultitask 93 0

D yeretal 2016 8 generative 93 3

300 21 α 0 3increased the m axim um outputlength to inputlength W e used a beam size of and

forboth W SJonly and the sem isupervised setting

O urresults in Table 4 show thatdespite the lack oftask specific tuning ourm odelperform s sur

prisingly w ell yielding betterresultsthan allpreviously reported m odelsw ith the exception ofthe

R ecurrentN euralN etw ork G ram m ar 8

In contrastto R N N sequence to sequence m odels 37 the Transform eroutperform sthe B erkeley

Parser 29 even w hen training only on the W SJtraining setof40K sentences

7 C on clu sion

In thisw ork w e presented the Transform er the firstsequence transduction m odelbased entirely on

attention replacing the recurrentlayersm ostcom m only used in encoderdecoderarchitecturesw ith

m ultiheaded self attention

Fortranslation tasks the Transform ercan be trained significantly fasterthan architectures based

on recurrentor convolutionallayers O n both W M T 2014 English to G erm an and W M T 2014

English to French translation tasks w e achieve a new state ofthe art In the form ertask ourbest

m odeloutperform seven allpreviously reported ensem bles

W e are excited aboutthe future ofattention based m odelsand plan to apply them to othertasks W e

plan to extend the Transform erto problem sinvolving inputand outputm odalitiesotherthan textand

to investigate local restricted attention m echanism s to efficiently handle large inputs and outputs

such asim ages audio and video M aking generation lesssequentialisanotherresearch goalsofours

The code w e used to train and evaluate our m odels is available at https github com

tensorflow tensor2tensor

W e are gratefulto N alK alchbrennerand Stephan G ouw s fortheirfruitfulA cknow ledgem ents

com m ents correctionsand inspiration

R eferen ces

arXiv preprint1 Jim m y LeiB a Jam ie R yan K iros and G eoffrey E H inton Layernorm alization

arXiv 1607 06450 2016

2 D zm itry B ahdanau K yunghyun C ho and Yoshua B engio N euralm achine translation by jointly

C oRRlearning to align and translate abs 1409 0473 2014

3 D enny B ritz A nna G oldie M inh Thang Luong and Q uoc V Le M assive exploration ofneural

C oRRm achine translation architectures abs 1703 03906 2017

4 Jianpeng C heng LiD ong and M irella Lapata Long shortterm m em ory netw orksform achine

arXiv preprintarXiv 1601 06733reading 2016

10

5 K yunghyun C ho B artvan M errienboer C aglarG ulcehre FethiB ougares H olgerSchw enk

and Yoshua B engio Learning phrase representationsusing rnn encoderdecoderforstatistical

C oRRm achine translation abs 1406 1078 2014

arXiv6 Francois C hollet X ception D eep learning w ith depthw ise separable convolutions

preprintarXiv 1610 02357 2016

7 Junyoung C hung ÇaglarG ülçehre K yunghyun C ho and Yoshua B engio Em piricalevaluation

C oRRofgated recurrentneuralnetw orkson sequence m odeling abs 1412 3555 2014

8 C hris D yer A dhiguna K uncoro M iguelB allesteros and N oah A Sm ith R ecurrentneural

Proc ofNAAC Lnetw ork gram m ars In 2016

9 JonasG ehring M ichaelA uli D avid G rangier D enisYarats and Yann N D auphin C onvolu

arXiv preprintarXiv 1705 03122v2tionalsequence to sequence learning 2017

arXiv preprint10 A lex G raves G enerating sequences w ith recurrent neural netw orks

arXiv 1308 0850 2013

11 K aim ing H e X iangyu Zhang Shaoqing R en and Jian Sun D eep residuallearning forim

Proceedings ofthe IEEE C onference on C om puter Vision and Patternage recognition In

Recognition pages770 778 2016

12 Sepp H ochreiter Yoshua B engio Paolo Frasconi and Jürgen Schm idhuber G radientflow in

recurrentnets the difficulty oflearning long term dependencies 2001

N euralcom putation13 Sepp H ochreiter and Jürgen Schm idhuber Long shortterm m em ory

9 8 1735 1780 1997

14 Zhongqiang H uang and M ary H arper Self training PC FG gram m arsw ith latentannotations

Proceedings ofthe 2009 C onference on Em piricalM ethods in N aturalacross languages In

Language Processing pages832 841 A C L A ugust2009

15 R afalJozefow icz O riolV inyals M ike Schuster N oam Shazeer and YonghuiW u Exploring

arXiv preprintarXiv 1602 02410the lim itsoflanguage m odeling 2016

Advancesin N eural16 Łukasz K aiserand Sam y B engio C an active m em ory replace attention In

Inform ation Processing System s N IPS 2016

InternationalC onference17 Łukasz K aiserand Ilya Sutskever N euralG PU slearn algorithm s In

on Learning Representations IC LR 2016

18 N alK alchbrenner Lasse Espeholt K aren Sim onyan A aron van den O ord A lex G raves and K o

arXivpreprintarXiv 1610 10099v2ray K avukcuoglu N euralm achinetranslation in lineartim e

2017

19 Yoon K im C arlD enton Luong H oang and A lexanderM R ush Structured attention netw orks

InternationalC onference on Learning RepresentationsIn 2017

IC LR20 D iederik K ingm a and Jim m y B a A dam A m ethod forstochastic optim ization In 2015

arXiv preprint21 O leksiiK uchaiev and B orisG insburg Factorization tricksforLSTM netw orks

arXiv 1703 10722 2017

22 Zhouhan Lin M inw ei Feng C icero N ogueira dos Santos M o Yu B ing X iang B ow en

arXiv preprintZhou and Yoshua B engio A structured self attentive sentence em bedding

arXiv 1703 03130 2017

23 M inh Thang Luong Q uoc V Le Ilya Sutskever O riolV inyals and Lukasz K aiser M ultitask

arXiv preprintarXiv 1511 06114sequence to sequence learning 2015

24 M inh Thang Luong H ieu Pham and C hristopherD M anning Effective approachesto attention

arXiv preprintarXiv 1508 04025based neuralm achine translation 2015

11

25 M itchellP M arcus M ary A nn M arcinkiew icz and B eatriceSantorini B uilding alargeannotated

C om putationallinguisticscorpusofenglish The penn treebank 19 2 313 330 1993

26 D avid M cC losky Eugene C harniak and M ark Johnson Effective self training forparsing In

Proceedingsofthe H um an Language Technology C onference ofthe NAAC L M ain C onference

pages152 159 A C L June 2006

27 A nkurParikh O scarTäckström D ipanjan D as and Jakob U szkoreit A decom posable attention

Em piricalM ethodsin N aturalLanguage Processingm odel In 2016

28 R om ain Paulus C aim ing X iong and R ichard Socher A deep reinforced m odelforabstractive

arXiv preprintarXiv 1705 04304sum m arization 2017

29 Slav Petrov Leon B arrett R om ain Thibaux and D an K lein Learning accurate com pact

Proceedings ofthe 21stInternationalC onference onand interpretable tree annotation In

C om putationalLinguistics and 44th AnnualM eeting ofthe AC L pages 433 440 A C L July

2006

arXiv30 O firPress and LiorW olf U sing the outputem bedding to im prove language m odels

preprintarXiv 1608 05859 2016

31 R ico Sennrich B arry H addow and A lexandra B irch N euralm achine translation ofrare w ords

arXiv preprintarXiv 1508 07909w ith subw ord units 2015

32 N oam Shazeer A zalia M irhoseini K rzysztofM aziarz A ndy D avis Q uoc Le G eoffrey H inton

and JeffD ean O utrageously large neuralnetw orks The sparsely gated m ixture of experts

arXiv preprintarXiv 1701 06538layer 2017

33 N itish Srivastava G eoffrey E H inton A lex K rizhevsky Ilya Sutskever and R uslan Salakhutdi

JournalofM achinenov D ropout a sim ple w ay to preventneuralnetw orksfrom overfitting

Learning Research 15 1 1929 1958 2014

34 Sainbayar Sukhbaatar A rthur Szlam Jason W eston and R ob Fergus End to end m em ory

netw orks In C C ortes N D Law rence D D Lee M Sugiyam a and R G arnett editors

Advancesin N euralInform ation Processing System s28 pages2440 2448 C urran A ssociates

Inc 2015

35 Ilya Sutskever O riolV inyals and Q uoc V V Le Sequence to sequence learning w ith neural

Advancesin N euralInform ation Processing System snetw orks In pages3104 3112 2014

36 C hristian Szegedy V incentVanhoucke Sergey Ioffe Jonathon Shlens and Zbigniew W ojna

C oRRR ethinking the inception architecture forcom putervision abs 1512 00567 2015

37 V inyals K aiser K oo Petrov Sutskever and H inton G ram m aras a foreign language In

Advancesin N euralInform ation Processing System s 2015

38 YonghuiW u M ike Schuster Zhifeng C hen Q uoc V Le M oham m ad N orouzi W olfgang

M acherey M axim K rikun Yuan C ao Q in G ao K lausM acherey etal G oogle sneuralm achine

arXiv preprinttranslation system B ridging the gap betw een hum an and m achine translation

arXiv 1609 08144 2016

39 Jie Zhou Y ing C ao X uguang W ang Peng Li and W eiX u D eep recurrentm odels w ith

C oRRfastforw ard connectionsforneuralm achine translation abs 1606 04199 2016

40 M uhua Zhu Yue Zhang W enliang C hen M in Zhang and Jingbo Zhu Fastand accurate

Proceedingsofthe 51stAnnualM eeting ofthe AC L Volum eshiftreduce constituentparsing In

1 Long Papers pages434 443 A C L A ugust2013

12

In p u t In p u t L a y e r5A tten tion V isu alization s

st

n ne o

n ita m

sy tat d gc ln si ri

rr te gnr u Seit e e d d dd d d9 se e si o n c

e s cit j k ri icr O a a aa a av sv w 0 ts i fog

i a ea a ow fnm E p p pp ppo a r rf a e 0 iop i eh h ht ns a

i t t m g p s m o v pI i s a o A h n l 2 t r m d

t t f tt rs s li ssy sI a e en en e gd 9 g ni i wi t tr a o o r

sii uv c ha ne 0 n o d d d d d dw Sh er iin ih t ot cep a n tc s t

t 0 a a a a a aiai n i ko e l Ofcs j sr h oa2 p p p p ppm

fs a r ioa va Ee m t r

dmp snm pim r

geA ev

ro

g

Figure 3 A n exam ple of the attention m echanism follow ing long distance dependencies in the

encoderself attention in layer5 of6 M any ofthe attention headsattend to a distantdependency of

the verb m aking com pleting the phrase m aking m ore difficult A ttentionshere show n only for

the w ord m aking D ifferentcolorsrepresentdifferentheads B estview ed in color

13

In p u t In p u t L a y e r 5

n

oit

gt na d

c nr lc oi Si it

e dle u sf nt a O

e l av r tw p io s sl ei yisi h e

h pEa e e e u p e r phs h nt u s

w n p b s j i w i mT L b i a b t w a m o

l t t t trl s s yse e e e ne gd nni i iw

tc s ilu a re ih b b n oo dSh wa

me u iui iw v b h atjf t aT L s

o nre Ow ia ps

hen iIn p u t In p u t L a y e r 5 pc Ei s

p l om

p

p

a

n

oit

gt na d

c nr lc oi Si it

e dle u sf nt a O

e l aw v r t p io s sl ei yisi eh

h pEe e ua e p e r phs hu s nt

w n p b s j i w i mT L b i a b t w a m o

l t t t trl s s s ye e e e ne gdn ni i iw

tc s ilu a re ih b b n oo dSha w

me u iu iiw v b h atjf t aT L s

o nre Ow ia ps

hen i pc Ei s

p l om

p

p

a

Figure 4 Tw o attention heads also in layer5 of6 apparently involved in anaphora resolution Top

Fullattentionsforhead 5 B ottom Isolated attentionsfrom justthe w ord its forattention heads5

and 6 N ote thatthe attentionsare very sharp forthisw ord

14

In p u t In p u t L a y e r 5

n

oit

gt na d

c nr lc oi Si it

e dle u sf nt a O

e l av r tw p io s sl ei yisi eh

h pEe e ua e p e r phs h nt u s

iL w n b p b i a s b j t i w w a m oT m

l t t t trl s s yse e e nee gd nni i iw

tc s ilu a re ih b b n oo dSha w

me u iu iiw v b h atjf t aT L s

o nre Ow ia ps

hen i pc EIn p u t In p u t L a y e r 5i s

p l om

p

p

a

n

oit

gt na d

c nr lc oi Si it

e dle u sf nt a O

e l av r tw p io s sl ei yisi eh

h pEe e ua e p e r phs h nt u s

iL w n b p b i a s b j t i w w a m oT m

l t t t trl s s yse e e e ne gd nni i iw

tc s ilu a re ih b b n oo dSha w

me u iu iiw v b h atjf t aT L s

o nre Ow ia ps

hen i pc Ei s

p l om

p

p

a

Figure 5 M any ofthe attention heads exhibitbehaviourthatseem s related to the structure ofthe

sentence W e give tw o such exam plesabove from tw o differentheadsfrom the encoderself attention

atlayer5 of6 The headsclearly learned to perform differenttasks

15