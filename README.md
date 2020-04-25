# projetRustS2_4IABD2


## Partie 1 : Rappels de Rust

1)

	En Rust, les références sont ce qui permet d’emprunter des valeurs. 
	Ces références ressemblent au fonctionnement des pointeurs, qui pointent vers la mémoire. 
	Une référence se fait grâce aux opérateurs & ou &mut mais aussi en utilisant ref et ref mut (mut permet de rendre la valeur mutable).
	Les références ont une durée de vie qui est donc la portée de la référence. 
	Une fois cette portée dépassée, l’emprunt n’est plus valable, et est donc retiré de la mémoire. 



2)

	Afin de déclarer ses propres types, il existe en Rust, deux principales façons de le faire:
	Grâce à Enum
	Grâce à Struct
	Toutefois on peut aussi se servir d’alias afin de déclarer des alias d’autres types et donc créer des types.

3)


	Rust se compile nativement grâce à LLVM.


4)

	L’espace maximum d’adressage d’un processus de 8 bits est de 2^8 - 1 = 255 soit en hexadécimal : FF


5)

	Un processus est comme un conteneur qui permet d’exécuter, de stocker et isoler des programmes en cours. 
	Un processus va faire tourner son ou ses programmes  séparément des autres.
	Pour cela il dispose de son propre espace de stockage, sa propre mémoire allouée. 
	Il représente la base du fonctionnement du système permettant les exécutions simultanées d’instructions grâce aux threads. 

Source: Mes cours de système.


## Partie 2 : Pratique -micro-shell

1)

	Pour compiler son programme il faut lancer la commande cargo build (bien entendu il faut se déplacer sur le programme au préalable).
	Puis, pour l'exécuter il faut lancer la commande: ./target/debug/micro-shell
	Si on souhaite compiler et exécuter avec une seule commande on peut utiliser: cargo run

## Partie 3 : Execution d'un micro-shell

4)

	Rust nous force à récupérer le statut du processus car il doit savoir quand le processus se termine afin de le transmettre au processus parent.
5)

	Pendant que son enfant s'exécute, le programme attend qu'il se termine pour collecter son statut afin de savoir quoi faire.
	
