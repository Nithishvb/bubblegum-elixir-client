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

  def create_tree_config("6PfweYuGyuakodXUHR3Lm3yTsxg2fzAsf74z6Mks7wv7", "6PfweYuGyuakodXUHR3Lm3yTsxg2fzAsf74z6Mks7wv7", 5, 5), do: :erlang.nif_error(:nif_not_loaded)

end
