(function() {var implementors = {};
implementors["polkadot_test_runtime"] = [{"text":"impl <a class=\"trait\" href=\"xcm_executor/traits/transact_asset/trait.TransactAsset.html\" title=\"trait xcm_executor::traits::transact_asset::TransactAsset\">TransactAsset</a> for <a class=\"struct\" href=\"polkadot_test_runtime/xcm_config/struct.DummyAssetTransactor.html\" title=\"struct polkadot_test_runtime::xcm_config::DummyAssetTransactor\">DummyAssetTransactor</a>","synthetic":false,"types":["polkadot_test_runtime::xcm_config::DummyAssetTransactor"]}];
implementors["xcm_builder"] = [{"text":"impl&lt;Matcher:&nbsp;<a class=\"trait\" href=\"xcm_executor/traits/matches_fungible/trait.MatchesFungible.html\" title=\"trait xcm_executor::traits::matches_fungible::MatchesFungible\">MatchesFungible</a>&lt;Currency::Balance&gt;, AccountIdConverter:&nbsp;<a class=\"trait\" href=\"xcm_executor/traits/conversion/trait.Convert.html\" title=\"trait xcm_executor::traits::conversion::Convert\">Convert</a>&lt;<a class=\"struct\" href=\"xcm/v1/multilocation/struct.MultiLocation.html\" title=\"struct xcm::v1::multilocation::MultiLocation\">MultiLocation</a>, AccountId&gt;, Currency:&nbsp;Currency&lt;AccountId&gt;, AccountId:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, CheckedAccount:&nbsp;Get&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;AccountId&gt;&gt;&gt; <a class=\"trait\" href=\"xcm_executor/traits/transact_asset/trait.TransactAsset.html\" title=\"trait xcm_executor::traits::transact_asset::TransactAsset\">TransactAsset</a> for <a class=\"struct\" href=\"xcm_builder/struct.CurrencyAdapter.html\" title=\"struct xcm_builder::CurrencyAdapter\">CurrencyAdapter</a>&lt;Currency, Matcher, AccountIdConverter, AccountId, CheckedAccount&gt;","synthetic":false,"types":["xcm_builder::currency_adapter::CurrencyAdapter"]},{"text":"impl&lt;Assets:&nbsp;Transfer&lt;AccountId&gt;, Matcher:&nbsp;<a class=\"trait\" href=\"xcm_executor/traits/matches_fungibles/trait.MatchesFungibles.html\" title=\"trait xcm_executor::traits::matches_fungibles::MatchesFungibles\">MatchesFungibles</a>&lt;Assets::AssetId, Assets::Balance&gt;, AccountIdConverter:&nbsp;<a class=\"trait\" href=\"xcm_executor/traits/conversion/trait.Convert.html\" title=\"trait xcm_executor::traits::conversion::Convert\">Convert</a>&lt;<a class=\"struct\" href=\"xcm/v1/multilocation/struct.MultiLocation.html\" title=\"struct xcm::v1::multilocation::MultiLocation\">MultiLocation</a>, AccountId&gt;, AccountId:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"xcm_executor/traits/transact_asset/trait.TransactAsset.html\" title=\"trait xcm_executor::traits::transact_asset::TransactAsset\">TransactAsset</a> for <a class=\"struct\" href=\"xcm_builder/struct.FungiblesTransferAdapter.html\" title=\"struct xcm_builder::FungiblesTransferAdapter\">FungiblesTransferAdapter</a>&lt;Assets, Matcher, AccountIdConverter, AccountId&gt;","synthetic":false,"types":["xcm_builder::fungibles_adapter::FungiblesTransferAdapter"]},{"text":"impl&lt;Assets:&nbsp;Mutate&lt;AccountId&gt;, Matcher:&nbsp;<a class=\"trait\" href=\"xcm_executor/traits/matches_fungibles/trait.MatchesFungibles.html\" title=\"trait xcm_executor::traits::matches_fungibles::MatchesFungibles\">MatchesFungibles</a>&lt;Assets::AssetId, Assets::Balance&gt;, AccountIdConverter:&nbsp;<a class=\"trait\" href=\"xcm_executor/traits/conversion/trait.Convert.html\" title=\"trait xcm_executor::traits::conversion::Convert\">Convert</a>&lt;<a class=\"struct\" href=\"xcm/v1/multilocation/struct.MultiLocation.html\" title=\"struct xcm::v1::multilocation::MultiLocation\">MultiLocation</a>, AccountId&gt;, AccountId:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, CheckAsset:&nbsp;Contains&lt;Assets::AssetId&gt;, CheckingAccount:&nbsp;Get&lt;AccountId&gt;&gt; <a class=\"trait\" href=\"xcm_executor/traits/transact_asset/trait.TransactAsset.html\" title=\"trait xcm_executor::traits::transact_asset::TransactAsset\">TransactAsset</a> for <a class=\"struct\" href=\"xcm_builder/struct.FungiblesMutateAdapter.html\" title=\"struct xcm_builder::FungiblesMutateAdapter\">FungiblesMutateAdapter</a>&lt;Assets, Matcher, AccountIdConverter, AccountId, CheckAsset, CheckingAccount&gt;","synthetic":false,"types":["xcm_builder::fungibles_adapter::FungiblesMutateAdapter"]},{"text":"impl&lt;Assets:&nbsp;Mutate&lt;AccountId&gt; + Transfer&lt;AccountId&gt;, Matcher:&nbsp;<a class=\"trait\" href=\"xcm_executor/traits/matches_fungibles/trait.MatchesFungibles.html\" title=\"trait xcm_executor::traits::matches_fungibles::MatchesFungibles\">MatchesFungibles</a>&lt;Assets::AssetId, Assets::Balance&gt;, AccountIdConverter:&nbsp;<a class=\"trait\" href=\"xcm_executor/traits/conversion/trait.Convert.html\" title=\"trait xcm_executor::traits::conversion::Convert\">Convert</a>&lt;<a class=\"struct\" href=\"xcm/v1/multilocation/struct.MultiLocation.html\" title=\"struct xcm::v1::multilocation::MultiLocation\">MultiLocation</a>, AccountId&gt;, AccountId:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, CheckAsset:&nbsp;Contains&lt;Assets::AssetId&gt;, CheckingAccount:&nbsp;Get&lt;AccountId&gt;&gt; <a class=\"trait\" href=\"xcm_executor/traits/transact_asset/trait.TransactAsset.html\" title=\"trait xcm_executor::traits::transact_asset::TransactAsset\">TransactAsset</a> for <a class=\"struct\" href=\"xcm_builder/struct.FungiblesAdapter.html\" title=\"struct xcm_builder::FungiblesAdapter\">FungiblesAdapter</a>&lt;Assets, Matcher, AccountIdConverter, AccountId, CheckAsset, CheckingAccount&gt;","synthetic":false,"types":["xcm_builder::fungibles_adapter::FungiblesAdapter"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()