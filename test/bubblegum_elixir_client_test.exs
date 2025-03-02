defmodule BubblegumElixirClientTest do
  use ExUnit.Case
  doctest BubblegumElixirClient

  test "greets the world" do
    assert BubblegumElixirClient.hello() == :world
  end
end
