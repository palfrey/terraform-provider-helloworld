terraform {
  required_providers {
    helloworld = {
      source = "palfrey/helloworld"
      version = "0.0.1"
    }
  }
}

provider "helloworld" {
    foo = "bar"
}

resource "helloworld_thing" "test" {
    bar = "baz"
}