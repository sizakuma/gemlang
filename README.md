<div display="flex"><img src="https://github.com/sizakuma/gemlang/assets/142700972/1094211b-3365-4d3b-9bf2-07103fcc19fe" width=250></div>

**Gem** is a scripting compiled programming language.

````gem
from "io" import { out }
from "math" import { max } as math

def Entity
var Entity:name -> String

var Entity:health -> i64
set Entity:health(self, new) { self:health = math:max(new, 0) }
signal set Entity:health {
	if self:health <= 0 {
		self:is_dead()
	}
}

fun Entity:new(self, name -> String) {
  self:name = name
}

def Player(Entity)
def Enemy(Entity)

const Entity:player = Player()

fun Entity:is_dead(self) {
  out:println("{name} is dead" % self)
}
````
