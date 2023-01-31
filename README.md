# apvs

1. Funktionsweise

Dieses Projekt benutzt eine Containerisierung innerhalb von Docker, um eine verteilte Anwendung zu realisieren, welche beliebigen Kontext mit Buildanweisungen erhalten kann
um aus diesen ein Image und einen Container zu erstellen und diesen dann auszuführen.

Dabei besteht das Projekt aus drei Teilen, die zusammen die Funktion komponieren. Die Kommunikation zwischen den einzelnen Teilen der Anwendung findet über Webserver mittels actix:web statt.

Im Prototypen ist der Client der einzige Teil, der direkt auf dem Hostsystem ausgeführt wird, die anderen zwei Teile werden in Docker-Containern ausgeführt, um eine Verteilung zu simulieren. 
Der Client selbst ist, wie der Rest der Anwendung in Rust geschrieben und diesem kann man einen Ordner übergeben, in dem erwartet wird, dass sich dort ein Dockerfile sowie zusätzlicher Kontext befindet, der für das erstellen des Images nötig ist.
Hierbei wird das Dockerfile erwartet, als könne man dies direkt lokal zum Ausführen eines Builds per "docker build -f <zuSendenderOrdnerAlsBuildContext/Dockerfile> <zuSendenderOrdnerAlsBuildContext>" nutzen. Hier wird dann der Kontext gezippt, und anschließend an den Receiver geschickt. Der gezippte Ordner wird derzeit auch auf dem System gelassen, zu debug-Zwecken.

Der Receiver ist sehr basic implementiert im Prototypen und stellt einen der Punkte dar, die am ehesten erweitert werden sollten. Hier würden die einzelnen worker (executors) gemanaged und der korrekte ausgewählt. Quasi der Master-Node. Für den Prototypen gibt es allerdings nur einen worker und an diesen wird die Anfrage entsprechend weitergeleitet.
Ebenfalls über actix:web mittels POST-Requests. Der Receiver selbst hat als eine kleine extra-Funktion noch eingebaut, dass er sich alle 700 Sekunden selbst abschaltet, wodurch er wegen der docker-compose-Konfiguration neugestarte wird. Ein kleiner Bonus für etwas Fehlertoleranz.

Der Executor übernimmt dann die Aufgabe, aus dem Kontext ein Docker-Image zu bauen, dafür muss dieses zuerst in einen Tarball umgeformt werden, damit die benutzte Docker-API diesen für den Image-Bau nutzen kann.
Der Container, der den Executor realisiert, basiert auch auf dem docker:dind image, um Zugriff auf Docker-Funktionen innerhalb des Containers zu haben.
Wenn das Image erfolgreich gebaut wurde, wird anhand dessen ein Container gestartet und auf dem Executor-Node ausgeführt. Hierbei können mehrere Container auf einem Executor ausgeführt werden und laufen unabhängig voneinander.

2. Benutzung
In der Präsentation wurde der Prototyp mittels dem Befehl "cargo run --bin client <zuSendenderOrdner>" benutzt. Hier wird Cargo nur gesagt, wo der EntryPoint der Anwendung ist, nämlich im client.rs und dann der entsprechende Ordner angegeben. Cargo ist allerdings lediglich der Paketmanager von Rust und bei einer fertigen Implementierung und einem Build nach "cargo build --bin client --release" kann man die entstehende .exe (/target/release/client.exe) beliebig verschieben und diese lässt sich dann einfach benutzen mit 
"client.exe <zuSendenderOrdner>". Die Features sind sehr basic für den Prototypen und die ausgeführten Container lassen sich nicht großartig konfigurieren, ist jedoch als proof-of-concept gedacht und zeigt, dass diese Realisierung möglich ist.
Der Kontext selbst wurde bei der Präsentation noch in einem bestimmten Muster (Dateinamen etc.) erwartet, wurde aber inzwischen erweitert und akzeptiert jetzt beliebigen Kontext, sofern dieser alle Dateien beinhaltet, die in dem entsprechenden Dockerfile referenziert werden.
