Commands:

new: creates a nre arpeggio-based project
  arguments: --project-name



arpeggio-cli new <project-name>
arpeggio-cli add boundedcontext <bounded-context-name>
arpeggio-cli add aggregate <aggregate-name> <bounded-context-name?>

arpeggio-cli add adapter controller
arpeggio-cli add adapter presenter

arpeggio-cli add application command
arpeggio-cli add application query
arpeggio-cli add application subscriber

arpeggio-cli add domain entity
arpeggio-cli add domain event
arpeggio-cli add domain repository
arpeggio-cli add domain service
arpeggio-cli add domain valueobject

arpeggio-cli add infrastructure repository <repository-name> <bounded-context-name> <aggregate-name>
arpeggio-cli add infrastructure service <service-name> <bounded-context-name> <aggregate-name>