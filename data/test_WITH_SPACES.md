Provided properattribution isprovided,G oogle hereby grantsperm ission to

reproduce the tablesand figuresin thispapersolely foruse in journalistic or

scholarly w orks.

A tten tio n Is A ll Y o u N eed

3

2

0

2

g

u

∗ ∗ ∗ ∗Ashish Vaswani Noam Shazeer NikiParm ar Jakob UszkoreitA

GoogleBrain GoogleBrain GoogleResearch GoogleResearch

2 avaswani@google.com noam@google.com nikip@google.com usz@google.com

∗ ∗† ∗Llion Jones Aidan N.Gom ez ŁukaszKaiser]

L GoogleResearch UniversityofToronto GoogleBrain

C lukaszkaiser@google.comllion@google.com aidan@cs.toronto.edu

.

s

∗‡IlliaPolosukhinc

[ illia.polosukhin@gmail.com

7

v A bstract

2

6

Thedominantsequencetransductionmodelsarebasedoncomplexrecurrentor7

convolutionalneuralnetworksthatincludean encoderand adecoder.Thebest3

performing modelsalso connecttheencoderand decoderthrough an attention0

. mechanism. W eproposeanew simplenetwork architecture,theTransformer,

6 basedsolelyonattentionmechanisms,dispensingwithrecurrenceandconvolutions

0 entirely. Experimentson two machine translation tasksshow these modelsto

7 besuperiorinqualitywhilebeingmoreparallelizableandrequiringsignificantly

1 lesstimeto train.Ourmodelachieves28.4 BLEU on theW M T 2014 English-

:

to-German translation task,improving overtheexisting bestresults,includingv

ensembles,byover2BLEU.OntheW M T 2014English-to-Frenchtranslationtask,i

ourmodelestablishesanew single-modelstate-of-the-artBLEU scoreof41.8afterX

trainingfor3.5daysoneightGPUs,asmallfractionofthetrainingcostsofther

bestmodelsfrom theliterature.W eshow thattheTransformergeneralizeswelltoa

othertasksbyapplyingitsuccessfullytoEnglishconstituencyparsingbothwith

largeandlimitedtrainingdata.

∗Equalcontribution.Listingorderisrandom.JakobproposedreplacingRNNswithself-attentionandstarted

theefforttoevaluatethisidea.Ashish,withIllia,designedandimplementedthefirstTransformermodelsand

hasbeencruciallyinvolvedineveryaspectofthiswork.Noam proposedscaleddot-productattention,multi-head

attentionandtheparameter-freepositionrepresentationandbecametheotherpersoninvolvedinnearlyevery

detail.Nikidesigned,implemented,tunedandevaluatedcountlessmodelvariantsinouroriginalcodebaseand

tensor2tensor.Llionalsoexperimentedwithnovelmodelvariants,wasresponsibleforourinitialcodebase,and

efficientinferenceandvisualizations.LukaszandAidanspentcountlesslongdaysdesigningvariouspartsofand

implementingtensor2tensor,replacingourearliercodebase,greatlyimprovingresultsandmassivelyaccelerating

ourresearch.

†WorkperformedwhileatGoogleBrain.

‡WorkperformedwhileatGoogleResearch.

31stConferenceonNeuralInformationProcessingSystems(NIPS2017),LongBeach,CA,USA.

1 Introduction

Recurrentneuralnetworks,longshort-term memory[13]andgatedrecurrent[7]neuralnetworks

inparticular,havebeenfirmlyestablishedasstateoftheartapproachesinsequencemodelingand

transduction problemssuch aslanguagemodeling and machinetranslation [35,2,5].Numerous

effortshavesincecontinuedtopushtheboundariesofrecurrentlanguagemodelsandencoder-decoder

architectures[38,24,15].

Recurrentmodelstypicallyfactorcomputationalongthesymbolpositionsoftheinputandoutput

sequences.Aligningthepositionstostepsincomputationtime,theygenerateasequenceofhidden

h h tstates ,asafunctionoftheprevioushiddenstate andtheinputforposition .Thisinherently-t t 1

sequentialnatureprecludesparallelizationwithintrainingexamples,whichbecomescriticalatlonger

sequencelengths,asmemoryconstraintslimitbatchingacrossexamples.Recentworkhasachieved

significantimprovementsincomputationalefficiencythroughfactorizationtricks[21]andconditional

computation[32],whilealsoimprovingmodelperformanceincaseofthelatter.Thefundamental

constraintofsequentialcomputation,however,remains.

Attentionmechanismshavebecomeanintegralpartofcompellingsequencemodelingandtransduc-

tionmodelsinvarioustasks,allowingmodelingofdependencieswithoutregardtotheirdistancein

theinputoroutputsequences[2,19].Inallbutafew cases[27],however,suchattentionmechanisms

areusedinconjunctionwitharecurrentnetwork.

InthisworkweproposetheTransformer,amodelarchitectureeschewingrecurrenceandinstead

relyingentirelyonanattentionmechanism todraw globaldependenciesbetweeninputandoutput.

TheTransformerallowsforsignificantlymoreparallelizationandcanreachanew stateoftheartin

translationqualityafterbeingtrainedforaslittleastwelvehoursoneightP100GPUs.

2 Background

ThegoalofreducingsequentialcomputationalsoformsthefoundationoftheExtendedNeuralGPU

[16],ByteNet[18]andConvS2S[9],allofwhichuseconvolutionalneuralnetworksasbasicbuilding

block,computinghiddenrepresentationsinparallelforallinputandoutputpositions.Inthesemodels,

thenumberofoperationsrequiredtorelatesignalsfrom twoarbitraryinputoroutputpositionsgrows

inthedistancebetweenpositions,linearlyforConvS2SandlogarithmicallyforByteNet.Thismakes

itmoredifficultto learn dependenciesbetween distantpositions[12].In theTransformerthisis

reducedtoaconstantnumberofoperations,albeitatthecostofreducedeffectiveresolutiondue

to averaging attention-weighted positions,an effectwecounteractwith M ulti-Head Attention as

describedinsection3.2.

Self-attention,sometimescalledintra-attentionisanattentionmechanism relatingdifferentpositions

ofasinglesequenceinordertocomputearepresentationofthesequence.Self-attentionhasbeen

usedsuccessfullyinavarietyoftasksincludingreadingcomprehension,abstractivesummarization,

textualentailmentandlearningtask-independentsentencerepresentations[4,27,28,22].

End-to-endmemorynetworksarebasedonarecurrentattentionmechanism insteadofsequence-

alignedrecurrenceandhavebeenshowntoperform wellonsimple-languagequestionansweringand

languagemodelingtasks[34].

To thebestofourknowledge,however,the Transformeristhefirsttransduction modelrelying

entirelyonself-attentiontocomputerepresentationsofitsinputandoutputwithoutusingsequence-

alignedRNNsorconvolution.Inthefollowingsections,wewilldescribetheTransformer,motivate

self-attentionanddiscussitsadvantagesovermodelssuchas[17,18]and[9].

3 M odelA rchitecture

M ostcompetitiveneuralsequencetransductionmodelshaveanencoder-decoderstructure[5,2,35].

(x ,...,x )Here,theencodermapsan inputsequenceofsymbolrepresentations to asequence1 n

= (z ,...,z )z zofcontinuousrepresentations . Given ,the decoderthen generatesan output1 n

(y ,...,y )sequence ofsymbolsoneelementatatime.Ateachstepthemodelisauto-regressive1 m

[10],consumingthepreviouslygeneratedsymbolsasadditionalinputwhengeneratingthenext.

2

Figure1:TheTransformer-modelarchitecture.

TheTransformerfollowsthisoverallarchitectureusingstackedself-attentionandpoint-wise,fully

connectedlayersforboththeencoderanddecoder,shownintheleftandrighthalvesofFigure1,

respectively.

3.1 Encoderand DecoderStacks

N = 6Theencoderiscomposed ofastack of identicallayers.Each layerhastwoEncoder:

sub-layers.Thefirstisamulti-headself-attentionmechanism,andthesecondisasimple,position-

wisefullyconnectedfeed-forwardnetwork.W eemployaresidualconnection[11]aroundeachof

thetwo sub-layers,followed by layernormalization [1].Thatis,theoutputofeach sub-layeris

LayerNorm (x + Sublayer(x)) Sublayer(x),where isthefunctionimplementedbythesub-layer

itself.Tofacilitatetheseresidualconnections,allsub-layersinthemodel,aswellastheembedding

d = 512layers,produceoutputsofdimension .model

N = 6Thedecoderisalsocomposedofastackof identicallayers.InadditiontothetwoDecoder:

sub-layersineachencoderlayer,thedecoderinsertsathirdsub-layer,whichperformsmulti-head

attentionovertheoutputoftheencoderstack.Similartotheencoder,weemployresidualconnections

aroundeachofthesub-layers,followedbylayernormalization.W ealsomodifytheself-attention

sub-layerin thedecoderstack to preventpositionsfrom attending to subsequentpositions.This

masking,combinedwithfactthattheoutputembeddingsareoffsetbyoneposition,ensuresthatthe

i ipredictionsforposition candependonlyontheknownoutputsatpositionslessthan .

3.2 Attention

Anattentionfunctioncanbedescribedasmappingaqueryandasetofkey-valuepairstoanoutput,

wherethequery,keys,values,andoutputareallvectors.Theoutputiscomputedasaweightedsum

3

ScaledDot-ProductAttention M ulti-HeadAttention

Figure2:(left)Scaled Dot-ProductAttention. (right)M ulti-Head Attention consistsofseveral

attentionlayersrunninginparallel.

ofthevalues,wheretheweightassignedtoeachvalueiscomputedbyacompatibilityfunctionofthe

querywiththecorrespondingkey.

3.2.1 Scaled Dot-ProductAttention

W ecallourparticularattention"ScaledDot-ProductAttention"(Figure2).Theinputconsistsof

d dqueriesandkeysofdimension ,andvaluesofdimension .W ecomputethedotproductsofthek v√

dquerywithallkeys,divideeachby ,andapplyasoftmaxfunctiontoobtaintheweightsonthek

values.

Inpractice,wecomputetheattentionfunctiononasetofqueriessimultaneously,packedtogether

Q K Vintoamatrix .Thekeysandvaluesarealsopackedtogetherintomatrices and .W ecompute

thematrixofoutputsas:

TQK

√Attention(Q,K ,V )= softm ax( )V (1)

dk

Thetwomostcommonlyusedattentionfunctionsareadditiveattention[2],anddot-product(multi-

plicative)attention.Dot-productattentionisidenticaltoouralgorithm,exceptforthescalingfactor

1√of .Additiveattentioncomputesthecompatibilityfunctionusingafeed-forwardnetworkwith

dk

asinglehiddenlayer.W hilethetwoaresimilarintheoreticalcomplexity,dot-productattentionis

muchfasterandmorespace-efficientinpractice,sinceitcanbeimplementedusinghighlyoptimized

matrixmultiplicationcode.

dW hileforsmallvaluesof thetwomechanismsperform similarly,additiveattentionoutperformsk

ddotproductattentionwithoutscalingforlargervaluesof [3].W esuspectthatforlargevaluesofk

d ,thedotproductsgrow largeinmagnitude,pushingthesoftmaxfunctionintoregionswhereithask

14 √extremelysmallgradients .Tocounteractthiseffect,wescalethedotproductsby .

dk

3.2.2 M ulti-Head Attention

dInsteadofperformingasingleattentionfunctionwith -dimensionalkeys,valuesandqueries,model

hwefounditbeneficialtolinearlyprojectthequeries,keysandvalues timeswithdifferent,learned

d d dlinearprojectionsto , and dimensions,respectively.Oneachoftheseprojectedversionsofk k v

dqueries,keysandvalueswethenperform theattentionfunctioninparallel,yielding -dimensionalv

4 q kToillustratewhythedotproductsgetlarge,assumethatthecomponentsof and areindependentrandom

dk∑·0 1 q k= qk 0 dvariableswithmean andvariance .Thentheirdotproduct, ,hasmean andvariance .

ii ki=1

4

outputvalues.Theseareconcatenated and onceagain projected,resulting in thefinalvalues,as

depictedinFigure2.

M ulti-headattentionallowsthemodeltojointlyattendtoinformationfrom differentrepresentation

subspacesatdifferentpositions.W ithasingleattentionhead,averaginginhibitsthis.

OM ultiHead(Q,K ,V )= Concat(head ,...,head )W

1 h

Q K V

head = Attention(QW ,K W ,V W )where i i ii

Q × × ×d d K d d V d dvk kmodel model modelR R R

∈ ∈ ∈W W WW heretheprojectionsareparametermatrices , ,i ii

×O hd dv modelR∈Wand .

h = 8In thiswork we employ parallelattention layers,orheads. Foreach ofthese we use

d = d = d /h = 64.Duetothereduceddimensionofeachhead,thetotalcomputationalcostk v model

issimilartothatofsingle-headattentionwithfulldimensionality.

3.2.3 ApplicationsofAttention in ourM odel

TheTransformerusesmulti-headattentioninthreedifferentways:

•In"encoder-decoderattention"layers,thequeriescomefrom thepreviousdecoderlayer,

andthememorykeysandvaluescomefrom theoutputoftheencoder.Thisallowsevery

positioninthedecodertoattendoverallpositionsintheinputsequence.Thismimicsthe

typicalencoder-decoderattention mechanismsin sequence-to-sequencemodelssuch as

[38,2,9].

•Theencodercontainsself-attentionlayers.Inaself-attentionlayerallofthekeys,values

andqueriescomefrom thesameplace,inthiscase,theoutputofthepreviouslayerinthe

encoder.Eachpositionintheencodercanattendtoallpositionsinthepreviouslayerofthe

encoder.

•Similarly,self-attentionlayersinthedecoderallow eachpositioninthedecodertoattendto

allpositionsinthedecoderuptoandincludingthatposition.W eneedtopreventleftward

informationflow inthedecodertopreservetheauto-regressiveproperty.W eimplementthis

-∞insideofscaleddot-productattentionbymaskingout(settingto )allvaluesintheinput

ofthesoftmaxwhichcorrespondtoillegalconnections.SeeFigure2.

3.3 Position-wiseFeed-Forward Networks

Inadditiontoattentionsub-layers,eachofthelayersinourencoderanddecodercontainsafully

connectedfeed-forwardnetwork,whichisappliedtoeachpositionseparatelyandidentically.This

consistsoftwolineartransformationswithaReLU activationinbetween.

FFN(x)= m ax(0,xW + b )W + b (2)1 1 2 2

W hilethelineartransformationsarethesameacrossdifferentpositions,theyusedifferentparameters

from layerto layer. Anotherway ofdescribing thisisastwo convolutionswith kernelsize 1.

d = 512Thedimensionality ofinputand outputis ,and theinner-layerhasdimensionalitymodel

d = 2048.ff

3.4 Em beddingsand Softm ax

Similarlytoothersequencetransductionmodels,weuselearnedembeddingstoconverttheinput

dtokensandoutputtokenstovectorsofdimension .W ealsousetheusuallearnedlineartransfor-model

mationandsoftmaxfunctiontoconvertthedecoderoutputtopredictednext-tokenprobabilities.In

ourmodel,wesharethesameweightmatrixbetweenthetwoembeddinglayersandthepre-softmax√

dlineartransformation,similarto[30].Intheembeddinglayers,wemultiplythoseweightsby .model

5

Table1: M aximum pathlengths,per-layercomplexityandminimum numberofsequentialoperations

n d kfordifferentlayertypes. isthesequencelength, istherepresentationdimension, isthekernel

rsizeofconvolutionsand thesizeoftheneighborhoodinrestrictedself-attention.

LayerType ComplexityperLayer Sequential M aximum PathLength

Operations

2·O(n d) O(1) O(1)Self-Attention

2·O(n d ) O(n) O(n)Recurrent

2· ·O(k n d ) O(1) O(log (n))Convolutional

k

· ·O(r n d) O(1) O(n/r)Self-Attention(restricted)

3.5 PositionalEncoding

Sinceourmodelcontainsnorecurrenceandnoconvolution,inorderforthemodeltomakeuseofthe

orderofthesequence,wemustinjectsomeinformationabouttherelativeorabsolutepositionofthe

tokensinthesequence.Tothisend,weadd"positionalencodings"totheinputembeddingsatthe

dbottomsoftheencoderanddecoderstacks.Thepositionalencodingshavethesamedimension model

astheembeddings,sothatthetwocanbesummed.Therearemanychoicesofpositionalencodings,

learnedandfixed[9].

Inthiswork,weusesineandcosinefunctionsofdifferentfrequencies:

2i/dmodelPE = sin(pos/10000 )

(pos,2i)

2i/dmodelPE = cos(pos/10000 )

(pos,2i+1)

pos iwhere isthepositionand isthedimension.Thatis,eachdimensionofthepositionalencoding

·2π 10000 2πcorrespondstoasinusoid.Thewavelengthsform ageometricprogressionfrom to .W e

chosethisfunctionbecausewehypothesizeditwouldallow themodeltoeasilylearntoattendby

k PErelativepositions,sinceforanyfixedoffset , canberepresentedasalinearfunctionofpos+k

PE .pos

W ealsoexperimentedwithusinglearnedpositionalembeddings[9]instead,andfoundthatthetwo

versionsproducednearlyidenticalresults(seeTable3row (E)).W echosethesinusoidalversion

becauseitmayallow themodeltoextrapolatetosequencelengthslongerthantheonesencountered

duringtraining.

4 W hy Self-A ttention

In thissection wecomparevariousaspectsofself-attention layersto therecurrentand convolu-

tionallayerscommonlyusedformappingonevariable-lengthsequenceofsymbolrepresentations

dR∈(x ,...,x ) (z ,...,z ) x ,zto anothersequenceofequallength ,with ,such asahidden

1 n 1 n i i

layerinatypicalsequencetransductionencoderordecoder.M otivatingouruseofself-attentionwe

considerthreedesiderata.

Oneisthetotalcomputationalcomplexityperlayer.Anotheristheamountofcomputationthatcan

beparallelized,asmeasuredbytheminimum numberofsequentialoperationsrequired.

Thethirdisthepathlengthbetweenlong-rangedependenciesinthenetwork.Learninglong-range

dependenciesisakeychallengeinmanysequencetransductiontasks.Onekeyfactoraffectingthe

abilitytolearnsuchdependenciesisthelengthofthepathsforwardandbackwardsignalshaveto

traverseinthenetwork.Theshorterthesepathsbetweenanycombinationofpositionsintheinput

andoutputsequences,theeasieritistolearnlong-rangedependencies[12].Hencewealsocompare

themaximum pathlengthbetweenanytwoinputandoutputpositionsinnetworkscomposedofthe

differentlayertypes.

AsnotedinTable1,aself-attentionlayerconnectsallpositionswithaconstantnumberofsequentially

O(n)executed operations,whereasarecurrentlayerrequires sequentialoperations.In termsof

computationalcomplexity,self-attentionlayersarefasterthanrecurrentlayerswhenthesequence

6

n dlength issmallerthan therepresentation dimensionality ,which ismostoften thecasewith

sentencerepresentationsusedbystate-of-the-artmodelsinmachinetranslations,suchasword-piece

[38]andbyte-pair[31]representations.Toimprovecomputationalperformancefortasksinvolving

rverylongsequences,self-attentioncouldberestrictedtoconsideringonlyaneighborhoodofsize in

theinputsequencecenteredaroundtherespectiveoutputposition.Thiswouldincreasethemaximum

O(n/r)pathlengthto .W eplantoinvestigatethisapproachfurtherinfuturework.

k < nA singleconvolutionallayerwithkernelwidth doesnotconnectallpairsofinputandoutput

O(n/k)positions.Doingsorequiresastackof convolutionallayersinthecaseofcontiguouskernels,

O(log (n))or in thecaseofdilated convolutions[18],increasing thelength ofthelongestpathsk

betweenanytwopositionsinthenetwork.Convolutionallayersaregenerallymoreexpensivethan

krecurrentlayers,by afactorof .Separableconvolutions[6],however,decreasethecomplexity

2· · ·O(k n d+ n d ) k = nconsiderably,to .Even with ,however,thecomplexityofaseparable

convolutionisequaltothecombinationofaself-attentionlayerandapoint-wisefeed-forwardlayer,

theapproachwetakeinourmodel.

Assidebenefit,self-attentioncouldyieldmoreinterpretablemodels.W einspectattentiondistributions

from ourmodelsandpresentanddiscussexamplesintheappendix.Notonlydoindividualattention

headsclearlylearntoperform differenttasks,manyappeartoexhibitbehaviorrelatedtothesyntactic

andsemanticstructureofthesentences.

5 Training

Thissectiondescribesthetrainingregimeforourmodels.

5.1 TrainingDataand Batching

W etrained on thestandard W M T 2014 English-German datasetconsisting ofabout4.5 million

sentencepairs.Sentenceswereencodedusingbyte-pairencoding[3],whichhasasharedsource-

targetvocabularyofabout37000tokens.ForEnglish-French,weusedthesignificantlylargerW M T

2014English-Frenchdatasetconsistingof36M sentencesandsplittokensintoa32000word-piece

vocabulary[38].Sentencepairswerebatchedtogetherbyapproximatesequencelength.Eachtraining

batchcontainedasetofsentencepairscontainingapproximately25000sourcetokensand25000

targettokens.

5.2 Hardwareand Schedule

W etrained ourmodelson onemachinewith 8 NVIDIA P100 GPUs.Forourbasemodelsusing

thehyperparametersdescribedthroughoutthepaper,eachtrainingsteptookabout0.4seconds.W e

trainedthebasemodelsforatotalof100,000stepsor12hours.Forourbigmodels,(describedonthe

bottom lineoftable3),steptimewas1.0seconds.Thebigmodelsweretrainedfor300,000steps

(3.5days).

5.3 Optim izer

-9β= 0.9 β= 0.98 ϵ= 10W eusedtheAdam optimizer[20]with , and .W evariedthelearning

1 2

rateoverthecourseoftraining,accordingtotheformula:

-0.5 - -0.5 1.5· ·lrate= d m in(step num ,step num warm up steps )

\_ \_ \_ (3)model

warm up stepsThiscorrespondstoincreasingthelearningratelinearlyforthefirst \_ trainingsteps,

anddecreasingitthereafterproportionallytotheinversesquarerootofthestepnumber.W eused

warm up steps= 4000\_ .

5.4 Regularization

W eemploythreetypesofregularizationduringtraining:

7

Table2:TheTransformerachievesbetterBLEU scoresthanpreviousstate-of-the-artmodelsonthe

English-to-GermanandEnglish-to-Frenchnewstest2014testsatafractionofthetrainingcost.

BLEU TrainingCost(FLOPs)

M odel

EN-DE EN-FR EN-DE EN-FR

ByteNet[18] 23.75

20·1.0 10Deep-Att+ PosUnk[39] 39.2

19 20· ·2.3 10 1.4 10GNM T + RL [38] 24.6 39.92

18 20· ·9.6 10 1.5 10ConvS2S [9] 25.16 40.46

19 20· ·2.0 10 1.2 10M oE [32] 26.03 40.56

20·8.0 10Deep-Att+ PosUnkEnsemble[39] 40.4

20 21· ·1.8 10 1.1 10GNM T + RL Ensemble[38] 26.30 41.16

19 21· ·7.7 10 1.2 10ConvS2S Ensemble[9] 26.36 41.29

18·3.3 10Transformer(basemodel) 27.3 38.1

19·2.3 10Transformer(big) 28.4 41.8

W eapplydropout[33]totheoutputofeachsub-layer,beforeitisaddedtotheResidualDropout

sub-layerinputandnormalized.Inaddition,weapplydropouttothesumsoftheembeddingsandthe

positionalencodingsinboththeencoderanddecoderstacks.Forthebasemodel,weusearateof

P = 0.1.drop

ϵ = 0.1Duringtraining,weemployedlabelsmoothingofvalue [36].ThisLabelSm oothing ls

hurtsperplexity,asthemodellearnstobemoreunsure,butimprovesaccuracyandBLEU score.

6 R esults

6.1 M achineTranslation

OntheW M T 2014English-to-Germantranslationtask,thebigtransformermodel(Transformer(big)

2.0inTable2)outperformsthebestpreviouslyreportedmodels(includingensembles)bymorethan

28.4BLEU,establishinganew state-of-the-artBLEU scoreof .Theconfigurationofthismodelis

3.5 8listedinthebottom lineofTable3.Trainingtook dayson P100GPUs.Evenourbasemodel

surpassesallpreviouslypublishedmodelsandensembles,atafractionofthetrainingcostofanyof

thecompetitivemodels.

41.0OntheW M T 2014English-to-Frenchtranslationtask,ourbigmodelachievesaBLEU scoreof ,

1/4outperformingallofthepreviouslypublishedsinglemodels,atlessthan thetrainingcostofthe

previousstate-of-the-artmodel.TheTransformer(big)modeltrainedforEnglish-to-Frenchused

P = 0.1 0.3dropoutrate ,insteadof .drop

Forthebasemodels,weusedasinglemodelobtainedbyaveragingthelast5checkpoints,which

werewrittenat10-minuteintervals.Forthebigmodels,weaveragedthelast20checkpoints.W e

4 α= 0.6usedbeam searchwithabeam sizeof andlengthpenalty [38].Thesehyperparameters

werechosenafterexperimentationonthedevelopmentset.W esetthemaximum outputlengthduring

50inferencetoinputlength+ ,butterminateearlywhenpossible[38].

Table2summarizesourresultsandcomparesourtranslationqualityandtrainingcoststoothermodel

architecturesfrom theliterature.W eestimatethenumberoffloatingpointoperationsusedtotraina

modelbymultiplyingthetrainingtime,thenumberofGPUsused,andanestimateofthesustained

5single-precisionfloating-pointcapacityofeachGPU .

6.2 M odelVariations

ToevaluatetheimportanceofdifferentcomponentsoftheTransformer,wevariedourbasemodel

indifferentways,measuringthechangeinperformanceonEnglish-to-Germantranslationonthe

5Weusedvaluesof2.8,3.7,6.0and9.5TFLOPSforK80,K40,M40andP100,respectively.

8

Table3:VariationsontheTransformerarchitecture.Unlistedvaluesareidenticaltothoseofthebase

model.AllmetricsareontheEnglish-to-Germantranslationdevelopmentset,newstest2013.Listed

perplexitiesareper-wordpiece,accordingtoourbyte-pairencoding,andshouldnotbecomparedto

per-wordperplexities.

train PPL BLEU params

N d d h d d P ϵk v drop lsmodel ff 6

×10steps (dev) (dev)

base 6 512 2048 8 64 64 0.1 0.1 100K 4.92 25.8 65

1 512 512 5.29 24.9

4 128 128 5.00 25.5

(A)

16 32 32 4.91 25.8

32 16 16 5.01 25.4

16 5.16 25.1 58

(B)

32 5.01 25.4 60

2 6.11 23.7 36

4 5.19 25.3 50

8 4.88 25.5 80

(C) 256 32 32 5.75 24.5 28

1024 128 128 4.66 26.0 168

1024 5.12 25.4 53

4096 4.75 26.2 90

0.0 5.77 24.6

0.2 4.95 25.5

(D)

0.0 4.67 25.3

0.2 5.47 25.7

(E) positionalembeddinginsteadofsinusoids 4.92 25.7

big 6 1024 4096 16 0.3 300K 2134.33 26.4

developmentset,newstest2013.W eusedbeam searchasdescribedintheprevioussection,butno

checkpointaveraging.W epresenttheseresultsinTable3.

InTable3rows(A),wevarythenumberofattentionheadsandtheattentionkeyandvaluedimensions,

keeping the amountofcomputation constant,asdescribed in Section 3.2.2. W hile single-head

attentionis0.9BLEU worsethanthebestsetting,qualityalsodropsoffwithtoomanyheads.

dInTable3rows(B),weobservethatreducingtheattentionkeysize hurtsmodelquality.Thisk

suggeststhatdetermining compatibility isnoteasy and thata more sophisticated compatibility

functionthandotproductmaybebeneficial.W efurtherobserveinrows(C)and(D)that,asexpected,

biggermodelsarebetter,anddropoutisveryhelpfulinavoidingover-fitting.Inrow (E)wereplaceour

sinusoidalpositionalencodingwithlearnedpositionalembeddings[9],andobservenearlyidentical

resultstothebasemodel.

6.3 English ConstituencyParsing

ToevaluateiftheTransformercangeneralizetoothertasksweperformedexperimentsonEnglish

constituencyparsing.Thistaskpresentsspecificchallenges:theoutputissubjecttostrongstructural

constraintsand issignificantly longerthan theinput. Furthermore,RNN sequence-to-sequence

modelshavenotbeenabletoattainstate-of-the-artresultsinsmall-dataregimes[37].

d = 1024W etraineda4-layertransformerwith ontheW allStreetJournal(W SJ)portionofthemodel

PennTreebank[25],about40K trainingsentences.W ealsotraineditinasemi-supervisedsetting,

usingthelargerhigh-confidenceandBerkleyParsercorporafrom withapproximately17M sentences

[37].W eusedavocabularyof16K tokensfortheW SJonlysettingandavocabularyof32K tokens

forthesemi-supervisedsetting.

W eperformedonlyasmallnumberofexperimentstoselectthedropout,bothattentionandresidual

(section5.4),learningratesandbeam sizeontheSection22developmentset,allotherparameters

remained unchanged from theEnglish-to-German basetranslation model.During inference,we

9

Table4:TheTransformergeneralizeswelltoEnglishconstituencyparsing(ResultsareonSection23

ofW SJ)

Parser Training W SJ23F1

Vinyals& Kaiserelal.(2014)[37] W SJonly,discriminative 88.3

Petrovetal.(2006)[29] W SJonly,discriminative 90.4

Zhuetal.(2013)[40] W SJonly,discriminative 90.4

Dyeretal.(2016)[8] W SJonly,discriminative 91.7

Transformer(4layers) W SJonly,discriminative 91.3

Zhuetal.(2013)[40] semi-supervised 91.3

Huang& Harper(2009)[14] semi-supervised 91.3

M cCloskyetal.(2006)[26] semi-supervised 92.1

Vinyals& Kaiserelal.(2014)[37] semi-supervised 92.1

Transformer(4layers) semi-supervised 92.7

Luongetal.(2015)[23] multi-task 93.0

Dyeretal.(2016)[8] generative 93.3

300 21 α= 0.3increasedthemaximum outputlengthtoinputlength+ .W eusedabeam sizeof and

forbothW SJonlyandthesemi-supervisedsetting.

Ourresultsin Table4 show thatdespitethelack oftask-specifictuning ourmodelperformssur-

prisinglywell,yieldingbetterresultsthanallpreviouslyreportedmodelswiththeexceptionofthe

RecurrentNeuralNetworkGrammar[8].

IncontrasttoRNN sequence-to-sequencemodels[37],theTransformeroutperformstheBerkeley-

Parser[29]evenwhentrainingonlyontheW SJtrainingsetof40K sentences.

7 C onclusion

Inthiswork,wepresentedtheTransformer,thefirstsequencetransductionmodelbasedentirelyon

attention,replacingtherecurrentlayersmostcommonlyusedinencoder-decoderarchitectureswith

multi-headedself-attention.

Fortranslation tasks,theTransformercan betrained significantly fasterthan architecturesbased

on recurrentorconvolutionallayers. On both W M T 2014 English-to-German and W M T 2014

English-to-Frenchtranslationtasks,weachieveanew stateoftheart.Intheformertaskourbest

modeloutperformsevenallpreviouslyreportedensembles.

W eareexcitedaboutthefutureofattention-basedmodelsandplantoapplythem toothertasks.W e

plantoextendtheTransformertoproblemsinvolvinginputandoutputmodalitiesotherthantextand

toinvestigatelocal,restrictedattentionmechanismstoefficientlyhandlelargeinputsandoutputs

suchasimages,audioandvideo.M akinggenerationlesssequentialisanotherresearchgoalsofours.

The code we used to train and evaluate our models is available athttps://github.com/

.tensorflow/tensor2tensor

W earegratefultoNalKalchbrennerandStephanGouwsfortheirfruitfulAcknowledgem ents

comments,correctionsandinspiration.

R eferences

arXivpreprint[1] JimmyLeiBa,JamieRyanKiros,andGeoffreyE Hinton.Layernormalization.

arXiv:1607.06450,2016.

[2] DzmitryBahdanau,KyunghyunCho,andYoshuaBengio.Neuralmachinetranslationbyjointly

CoRRlearningtoalignandtranslate. ,abs/1409.0473,2014.

[3] DennyBritz,AnnaGoldie,M inh-ThangLuong,andQuocV.Le.M assiveexplorationofneural

CoRRmachinetranslationarchitectures. ,abs/1703.03906,2017.

[4] JianpengCheng,LiDong,andM irellaLapata.Longshort-term memory-networksformachine

arXivpreprintarXiv:1601.06733reading. ,2016.

10

[5] KyunghyunCho,BartvanM errienboer,CaglarGulcehre,FethiBougares,HolgerSchwenk,

andYoshuaBengio.Learningphraserepresentationsusingrnnencoder-decoderforstatistical

CoRRmachinetranslation. ,abs/1406.1078,2014.

arXiv[6] FrancoisChollet. Xception:Deep learning with depthwiseseparableconvolutions.

preprintarXiv:1610.02357,2016.

[7] JunyoungChung,ÇaglarGülçehre,KyunghyunCho,andYoshuaBengio.Empiricalevaluation

CoRRofgatedrecurrentneuralnetworksonsequencemodeling. ,abs/1412.3555,2014.

[8] ChrisDyer,AdhigunaKuncoro,M iguelBallesteros,and Noah A.Smith.Recurrentneural

Proc.ofNAACLnetworkgrammars.In ,2016.

[9] JonasGehring,M ichaelAuli,DavidGrangier,DenisYarats,andYannN.Dauphin.Convolu-

arXivpreprintarXiv:1705.03122v2tionalsequencetosequencelearning. ,2017.

arXiv preprint[10] Alex Graves. Generating sequences with recurrent neural networks.

arXiv:1308.0850,2013.

[11] Kaiming He,Xiangyu Zhang,Shaoqing Ren,and Jian Sun. Deep residuallearning forim-

ProceedingsoftheIEEE Conferenceon ComputerVision and Patternagerecognition. In

Recognition,pages770-778,2016.

[12] SeppHochreiter,YoshuaBengio,PaoloFrasconi,andJürgenSchmidhuber.Gradientflow in

recurrentnets:thedifficultyoflearninglong-term dependencies,2001.

Neuralcomputation[13] Sepp Hochreiterand Jürgen Schmidhuber. Long short-term memory. ,

9(8):1735-1780,1997.

[14] ZhongqiangHuangandM aryHarper.Self-trainingPCFG grammarswithlatentannotations

Proceedingsofthe2009ConferenceonEmpiricalM ethodsinNaturalacrosslanguages.In

LanguageProcessing,pages832-841.ACL,August2009.

[15] RafalJozefowicz,OriolVinyals,M ikeSchuster,Noam Shazeer,andYonghuiW u.Exploring

arXivpreprintarXiv:1602.02410thelimitsoflanguagemodeling. ,2016.

AdvancesinNeural[16] ŁukaszKaiserandSamyBengio.Canactivememoryreplaceattention? In

InformationProcessingSystems,(NIPS),2016.

InternationalConference[17] ŁukaszKaiserandIlyaSutskever.NeuralGPUslearnalgorithms.In

onLearningRepresentations(ICLR),2016.

[18] NalKalchbrenner,LasseEspeholt,KarenSimonyan,AaronvandenOord,AlexGraves,andKo-

arXivpreprintarXiv:1610.10099v2rayKavukcuoglu.Neuralmachinetranslationinlineartime. ,

2017.

[19] YoonKim,CarlDenton,LuongHoang,andAlexanderM .Rush.Structuredattentionnetworks.

InternationalConferenceonLearningRepresentationsIn ,2017.

ICLR[20] DiederikKingmaandJimmyBa.Adam:A methodforstochasticoptimization.In ,2015.

arXivpreprint[21] OleksiiKuchaievandBorisGinsburg.FactorizationtricksforLSTM networks.

arXiv:1703.10722,2017.

[22] Zhouhan Lin,M inweiFeng,Cicero Nogueira dos Santos,M o Yu,Bing Xiang,Bowen

arXivpreprintZhou,and YoshuaBengio.A structured self-attentivesentenceembedding.

arXiv:1703.03130,2017.

[23] M inh-ThangLuong,QuocV.Le,IlyaSutskever,OriolVinyals,andLukaszKaiser.M ulti-task

arXivpreprintarXiv:1511.06114sequencetosequencelearning. ,2015.

[24] M inh-ThangLuong,HieuPham,andChristopherD M anning.Effectiveapproachestoattention-

arXivpreprintarXiv:1508.04025basedneuralmachinetranslation. ,2015.

11

[25] M itchellPM arcus,M aryAnnM arcinkiewicz,andBeatriceSantorini.Buildingalargeannotated

Computationallinguisticscorpusofenglish:Thepenntreebank. ,19(2):313-330,1993.

[26] DavidM cClosky,EugeneCharniak,andM arkJohnson.Effectiveself-trainingforparsing.In

ProceedingsoftheHumanLanguageTechnologyConferenceoftheNAACL,M ainConference,

pages152-159.ACL,June2006.

[27] AnkurParikh,OscarTäckström,DipanjanDas,andJakobUszkoreit.A decomposableattention

EmpiricalM ethodsinNaturalLanguageProcessingmodel.In ,2016.

[28] RomainPaulus,CaimingXiong,andRichardSocher.A deepreinforcedmodelforabstractive

arXivpreprintarXiv:1705.04304summarization. ,2017.

[29] Slav Petrov,Leon Barrett,Romain Thibaux,and Dan Klein. Learning accurate,compact,

Proceedingsofthe21stInternationalConferenceonand interpretabletreeannotation. In

ComputationalLinguisticsand44thAnnualM eetingoftheACL,pages433-440.ACL,July

2006.

arXiv[30] OfirPressandLiorW olf.Usingtheoutputembeddingtoimprovelanguagemodels.

preprintarXiv:1608.05859,2016.

[31] RicoSennrich,BarryHaddow,andAlexandraBirch.Neuralmachinetranslationofrarewords

arXivpreprintarXiv:1508.07909withsubwordunits. ,2015.

[32] Noam Shazeer,AzaliaM irhoseini,KrzysztofM aziarz,AndyDavis,QuocLe,GeoffreyHinton,

and JeffDean.Outrageously largeneuralnetworks:Thesparsely-gated mixture-of-experts

arXivpreprintarXiv:1701.06538layer. ,2017.

[33] NitishSrivastava,GeoffreyE Hinton,AlexKrizhevsky,IlyaSutskever,andRuslanSalakhutdi-

JournalofM achinenov.Dropout:asimplewaytopreventneuralnetworksfrom overfitting.

LearningResearch,15(1):1929-1958,2014.

[34] SainbayarSukhbaatar,ArthurSzlam,Jason W eston,and Rob Fergus. End-to-end memory

networks.In C.Cortes,N.D.Lawrence,D.D.Lee,M .Sugiyama,and R.Garnett,editors,

AdvancesinNeuralInformationProcessingSystems28,pages2440-2448.CurranAssociates,

Inc.,2015.

[35] IlyaSutskever,OriolVinyals,andQuocVV Le.Sequencetosequencelearningwithneural

AdvancesinNeuralInformationProcessingSystemsnetworks.In ,pages3104-3112,2014.

[36] ChristianSzegedy,VincentVanhoucke,SergeyIoffe,JonathonShlens,andZbigniew W ojna.

CoRRRethinkingtheinceptionarchitectureforcomputervision. ,abs/1512.00567,2015.

[37] Vinyals& Kaiser,Koo,Petrov,Sutskever,andHinton.Grammarasaforeignlanguage.In

AdvancesinNeuralInformationProcessingSystems,2015.

[38] YonghuiW u,M ike Schuster,Zhifeng Chen,Quoc V Le,M ohammad Norouzi,W olfgang

M acherey,M axim Krikun,YuanCao,QinGao,KlausM acherey,etal.Google'sneuralmachine

arXivpreprinttranslationsystem:Bridgingthegapbetweenhumanandmachinetranslation.

arXiv:1609.08144,2016.

[39] Jie Zhou,Ying Cao,Xuguang W ang,Peng Li,and W eiXu. Deep recurrentmodelswith

CoRRfast-forwardconnectionsforneuralmachinetranslation. ,abs/1606.04199,2016.

[40] M uhuaZhu,YueZhang,W enliang Chen,M in Zhang,and Jingbo Zhu. Fastand accurate

Proceedingsofthe51stAnnualM eetingoftheACL(Volumeshift-reduceconstituentparsing.In

1:LongPapers),pages434-443.ACL,August2013.

12

In p u t-In p u t L a y e r5A ttention V isualizations

st

n ne o

n ita m

sy tat >d gc ln si ri

> > >> > >rr te gnr u Seit e e d d dd d d9 se e si o

n ce s cit j k ri icr O a a aa a av sv w 0 ts i f

ogi a ea a ow fnm E p p pp ppo a r rf a e 0 iop i eh h ht

ns ai t t m g p s m o v p < < < <I i s a o A h n l 2 t r m d . < < <

t .t f tt rs s li ssy sI a e en en e gd 9 g n > > > > > > >i i wi t tr a o o r

sii uv c ha ne 0 n o d d d d d dw Sh er iin ih t ot cep a n tc s t

t 0 a a a a a aiai n i ko e l Ofcs j sr h oa2 p p p p ppm

fs a r ioa va Ee m t << < < < <r

dmp sn <m pim r

geA ev

ro

g

Figure3:An exampleoftheattention mechanism following long-distancedependenciesin the

encoderself-attentioninlayer5of6.M anyoftheattentionheadsattendtoadistantdependencyof

theverb'making',completingthephrase'making...moredifficult'.Attentionshereshownonlyfor

theword'making'.Differentcolorsrepresentdifferentheads.Bestviewedincolor.

13

In p u t-In p u t L a y e r5

n

oit

gt >na d

c n >r lc oi Si it

e dle u sf nt a O

e l av r tw p io s sl ei yisi h e

h pEa e e e u p e r phs h nt u s

w n p b s j i w , i m <T L b , i a b - t w a m o . <

l , , .t t t t-rl s s yse e e e ne gd nn > >i i iw

tc s ilu a re ih b b n oo dSh wa

me u iui iw v b h atjf t aT L s

o nre Ow ia ps

hen iIn p u t-In p u t L a y e r5 pc E <i s

p l o <m

p

p

a

n

oit

gt >na d

c n >r lc oi Si it

e dle u sf nt a O

e l aw v r t p io s sl ei yisi eh

h pEe e ua e p e r phs hu s nt

w n p b s j i w , i m <T L b , i a b - t w a m o . <

l , , .t t t t-rl s s s ye e e e ne gdn n > >i i iw

tc s ilu a re ih b b n oo dSha w

me u iu iiw v b h atjf t aT L s

o nre Ow ia ps

hen i pc E <i s

p l o <m

p

p

a

Figure4:Twoattentionheads,alsoinlayer5of6,apparentlyinvolvedinanaphoraresolution.Top:

Fullattentionsforhead5.Bottom:Isolatedattentionsfrom justtheword'its'forattentionheads5

and6.Notethattheattentionsareverysharpforthisword.

14

In p u t-In p u t L a y e r5

n

oit

gt >na d

c n >r lc oi Si it

e dle u sf nt a O

e l av r tw p io s sl ei yisi eh

h pEe e ua e p e r phs h nt u s

i <L w n b p , b i a s b j - t i w w a , m o <T m .

l ., ,t t t t-rl s s yse e e nee gd nn > >i i iw

tc s ilu a re ih b b n oo dSha w

me u iu iiw v b h atjf t aT L s

o nre Ow ia ps

hen i pc EIn p u t-In p u t L a y e r5 <i s

p l o <m

p

p

a

n

oit

gt >na d

c n >r lc oi Si it

e dle u sf nt a O

e l av r tw p io s sl ei yisi eh

h pEe e ua e p e r phs h nt u s

i <L w n b p , b i a s b j - t i w w a , m o <T m .

l ., ,t t t t-rl s s yse e e e ne gd nn > >i i iw

tc s ilu a re ih b b n oo dSha w

me u iu iiw v b h atjf t aT L s

o nre Ow ia ps

hen i pc E <i s

p l o <m

p

p

a

Figure5:M anyoftheattentionheadsexhibitbehaviourthatseemsrelatedtothestructureofthe

sentence.W egivetwosuchexamplesabove,from twodifferentheadsfrom theencoderself-attention

atlayer5of6.Theheadsclearlylearnedtoperform differenttasks.

15