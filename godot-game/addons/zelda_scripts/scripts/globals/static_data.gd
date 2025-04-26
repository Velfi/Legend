extends Node

const game_data = {
	"stat_names": [
		"Guile",
		"Gumption",
		"Grandeur",
		"Genius",
		"Gusto",
	],
	"skill_names": [
		"Filing",
		"Fillibustering",
		"Contacting",
		"Convincing",
		"Estimating",
		"Empathizing",
		"Avoiding",
		"Moderating",
		"Organizing",
		"Gambling",
		"Prototyping",
		"Productionalizing",
	],
	"item_types": [
		"scrolls",
		"spellbooks",
		"wands",
		"potions",
		"reagents",
		"axes", "swords", "staves", "spears", "maces", "polearms",
		"leather", "chain", "plate", "cloth", "jewels",
		"treasure",
		"food"
	],
	"items": {
		"cake": {
			"item_type": "food",
			"item_name": "birthday cake",
		}
	},
	"weapons": [
		{
			"kind": "sword",
			"skillReq": 1,
			"rarity": "common",
			"name": "Shortsword",
			"diceroll": "2d4",
			"description": "A common sword. Sharp and gets the job done."
		},
		{
			"kind": "axe",
			"skillReq": 1,
			"rarity": "common",
			"name": "Woodcutter's Axe",
			"diceroll": "2d3+1",
			"description": "More of a tool than a weapon. Still dangerous."
		}
	],
	"trainers": [
		{
			"trainerName": "Pablo",
	 		"trainerTier": "expert",
			"skill": "Prototyping",
			"otherKnownTrainers": [],
		},
		{
			"trainerName": "Annalisa",
	 		"trainerTier": "master",
			"skill": "Filing",
			"otherKnownTrainers": ["Clack"],
		},
		{
			"trainerName": "Rufus",
	 		"trainerTier": "grandmaster",
			"skill": "Fillibustering",
			"otherKnownTrainers": [],
		},
		{
			"trainerName": "Clack",
	 		"trainerTier": "expert",
			"skill": "Contacting",
			"otherKnownTrainers": ["Annalisa"],
		},
		{
			"trainerName": "Senzo",
	 		"trainerTier": "expert",
			"skill": "Convincing",
			"otherKnownTrainers": [],
		},
		{
			"trainerName": "Esther",
	 		"trainerTier": "expert",
			"skill": "Estimating",
			"otherKnownTrainers": [],
		},
		{
			"trainerName": "Hattori",
	 		"trainerTier": "expert",
			"skill": "Empathizing",
			"otherKnownTrainers": [],
		},
	],
	"spellbooks": [
		{
			"kind": "dark",
			"name": "Toxic Cloud",
			"skillReq": 1,
			"description": "Attack an enemy by projecting a cloud of toxic gas at them.",
			"spellEffectId": 0
		},
		{
			"kind": "light",
			"name": "Bless",
			"skillReq": 1,
			"description": "Bless a party member, temporarily improving their abilities.",
			"spellEffectId": 1
		},
		{
			"kind": "fire",
			"name": "Light Torch",
			"skillReq": 1,
			"description": "Conjure a torchlight to illuminate your surroundings.",
			"spellEffectId": 2
		},
		{
			"kind": "water",
			"name": "Water Walking",
			"skillReq": 1,
			"description": "Enable your party to walk on water for a limited time.",
			"spellEffectId": 3
		},
		{
			"kind": "earth",
			"name": "Slow",
			"skillReq": 1,
			"description": "Cause an enemy to move and attack slower for a limited time.",
			"spellEffectId": 4
		},
		{
			"kind": "air",
			"name": "Wizard's Eye",
			"skillReq": 1,
			"description": "Reveal nearby enemies, treasures, doors, and mechanisms.",
			"spellEffectId": 5
		},
	],
	"conversations": {
		"convo_a": {
			"name": "Amy"
		},
		"convo_b": {
			"name": "Bill"
		},
		"convo_c": {
			"name": "Christy"
		},
		"convo_d": {
			"name": "Dave"
		},
		"convo_e": {
			"name": "Ella"
		},
		"convo_f": {
			"name": "Frank"
		},
		"convo_g": {
			"name": "Gigi"
		},
		"convo_h": {
			"name": "Harry"
		},
		"convo_i": {
			"name": "Irene"
		},
		"convo_j": {
			"name": "Joe"
		},
		"convo_k": {
			"name": "Karen"
		},
		"convo_l": {
			"name": "Leland"
		},
		"convo_m": {
			"name": "Marjorie"
		},
		"convo_n": {
			"name": "Nick"
		},
		"convo_o": {
			"name": "Olive"
		},
		"convo_p": {
			"name": "Peter"
		},
		"convo_q": {
			"name": "Q"
		},
		"convo_r": {
			"name": "Ron"
		},
		"convo_s": {
			"name": "Sandy"
		},
		"convo_t": {
			"name": "Torbjorn"
		}
	}
}

const DEFAULT_TEMPLE_DATA = {
	"temple_name": "An empty temple",
	"priest_name": "nobody",
	"tooltip": "An empty temple"
}

const DEFAULT_SHOP_DATA = {
	"shop_name": "An empty storefront",
	"tooltip": "An empty storefront",
	"merchant_name": "No one here",
	"can_identify": [],
	"can_purchase": [],
}


const DEFAULT_CONVERSATION_DATA = {
	"conversation_name": "An empty house",
	"speaker_name": "No one here",
	"tooltip": "An empty house",
}

const DEFAULT_ITEM_DATA = {
	"item_name": "An inexplicable object",
	"item_type": "unknown",
}
