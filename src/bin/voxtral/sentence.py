import sys
import spacy
from spacy.lang.fr.examples import sentences 
from utillc import *
import matplotlib.pyplot as plt
print_everything()
md = 'fr_core_news_sm'
md = "fr_dep_news_trf"
md = "fr_core_news_lg"
md = "fr_core_news_md"
nlp = spacy.load(md)

file = "../../../Loti-prime-jeunesse.txt"
with open(file,"r") as f:
    text = f.read()

text1 = """
Il y faisait toujours frais et on s’y tenait beaucoup, au grand calme, par les après-midi brûlants de l’été. De telles conceptions de l’ameublement déroutaient les bonnes dames d’alentour, qui possédaient en général des petits salons conventionnels, décorés dans le haut style des 96 tapissiers de Rochefort ou de Saintes ; mais elles sentaient là peut-être un je ne sais quoi indéfinissable qui les dépassait.


Et je ne puis me rappeler sans sourire cette appréciation, qui me fut énoncée un jour par une vieille paysanne du voisinage : « Vous creyez que je vois point qu’ol est ine grande dame, votre sœur !

52 X Mon frère, qui était toujours mon conseiller intime et secret, ne semblait pas prendre au tragique mes insuccès en littérature scolaire, et voici, sur son papier mince, jauni par le temps, l’exposé de ses idées là-dessus, tel que je le retrouve dans une de ses lettres de décembre 1864, – mêlé du reste à la description de l’une des pluies torrentielles de là-bas inondant les immenses palmes de son jardin, dans son île basse et baignée d’eau chaude à l’embouchure du Mékong : « J’y vois à peine pour t’écrire, mon petit frère chéri, tant il fait sombre en ce moment dans ma pauvre case en bambou ; c’est le déluge biblique qui tombe sur notre île de Poulo-Condor. (Cette case, comme il l’appelait, je la savais par cœur, tant il me l’avait décrite, avec même des plans à l’appui ; je connaissais aussi bien que lui-même 53 le gîte de Shao, son petit esclave annamite, le gîte de ses chevaux, celui de ses chiens, et le chai où l’on rencontrait toujours des serpents.) Vois-tu, rien chez nous ne ressemble à des orages pareils ; même ceux qui ont le bon esprit de se déchaîner sur la Limoise le jeudi soir, à point pour t’empêcher de rentrer à Rochefort, ne peuvent t’en donner aucune idée ; ce sont des seaux d’eau lancés à tour de bras contre mon toit ; les belles plantes, les belles fleurs de mon jardin sont couchées comme par des coups de cravache ; j’ai autour de ma case des palmes d’au moins cinq mètres de long qui se penchent pour déverser des cascades, et ma chienne Mirette, qui croit à la fin du monde, est venue se blottir toute mouillée entre mes jambes. Je ne te promets pas de te ramener Shao, car il est en train de devenir sacripant ; mais quant à Mirette, celle-là, attends-toi bien à la voir arriver au printemps avec moi, et recommande, je t’en prie, à M. Souris de ne pas lui crever les yeux. » 
"""

	
text = text.replace("\n", " ")
#EKOX(text)
tokens = nlp(text)

#f = lambda ix : str(ix[0]) + " : " + str(ix[1])
f = lambda ix : str(ix[1])

sents = list(map(str, tokens.sents))
ll = [ len(str(sent)) for sent in sents]
#plt.hist(ll, bins=100); plt.show()


rr, i, s, L = [], 1, "", 500
for i, se in enumerate(sents) :
	if len(s) > L :
		rr.append(s)
		s = se
	else :
		s = s + " " + se
#EKOX(len(rr))
		
ll = [ len(str(sent)) for sent in rr]
#plt.hist(ll, bins=100); plt.show()

def cut(x) :
	l = len(x)
	xx =  [x] if l < 500 else [ x[:l//2], x[l//2:]]
	return xx

rrr = list(map(cut, rr))
rrr = [ e for s in rrr for e in s]

rrr = list(map(cut, rr))
rrr = [ e for s in rrr for e in s]

ll = [ len(str(sent)) for sent in rrr]
#plt.hist(ll, bins=100); plt.show()

for i,sent in enumerate(rrr):
    print(str(sent).strip())


#EKOX("\n".join(map(f,enumerate(list(tokens.sents)))))
sys.exit(0)
for sent in tokens.sents:
    print(sent.string.strip())


tokenizer = nltk.data.load('tokenizers/punkt/english.pickle')


from nltk.tokenize import sent_tokenize



EKOX(tokenize.sent_tokenize())
	


