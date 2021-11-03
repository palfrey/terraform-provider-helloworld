terraform {
  required_providers {
    helloworld = {
      source  = "examplecorp/helloworld"
      version = ">= 1.0"
    }
  }
}

provider "helloworld" {
    foo = "bar"
}

resource "helloworld_thing" "test" {
    bar = "baz"
}