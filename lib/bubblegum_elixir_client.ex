defmodule BubblegumElixirClient do
  use Rustler, otp_app: :bubblegum_sdk, crate: "bubblegumsdk_native"
  @moduledoc """
  Documentation for `BubblegumElixirClient`.
  """

  @doc """
  Hello world.

  ## Examples

      iex> BubblegumElixirClient.hello()
      :world

  """
  def hello do
    :world
  end

  def add(), do: :erlang.nif_error(:nif_not_loaded)

end
