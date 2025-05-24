import init, { calc_js } from './pkg/ash_calc_app';
import "./style.css";

async function run() {
  await init();
  console.log("WASM module initialized.");

  document.getElementById("calc-button")!.addEventListener("click", () => {
    const params = getParamsFromUI();
    const result = calc_js(params, []);
    displayResult(result);
  });
}

function getParamsFromUI() {
  const getVal = (id: string) => parseFloat((document.getElementById(id) as HTMLInputElement).value);
  const getCheck = (id: string) => (document.getElementById(id) as HTMLInputElement).checked;

  const atkRatio = getVal("atk_ratio") / 100;
  const intimidationRatio = getVal("intimidation_ratio") / 100;
  const baseAtk = Number((getVal("atk") / (1 + atkRatio)).toFixed(0));
  const baseIntimidation = Math.ceil(getVal("intimidation") / (1 + intimidationRatio) * 10) / 10;

  return {
    attacker: {
      base_atk: baseAtk,
      atk_ratio: atkRatio,
      def_penetration_rate: getVal("def_penetration_rate") / 100,
      base_intimidation: baseIntimidation,
      intimidation_ratio: intimidationRatio,
      damage_ratio: getVal("damage_ratio"),
      crit_rate: getVal("crit_rate") / 100,
      crit_damage_ratio: getVal("crit_damage_ratio") / 100,
      bad_condition_bonus_rate: getVal("bad_condition_bonus_rate") / 100,
      hp_high_bonus_rate: getVal("hp_high_bonus_rate") / 100,
      hp_low_bonus_rate: getVal("hp_low_bonus_rate") / 100,
      skill_bonus_rate: getVal("skill_bonus_rate") / 100,
      vs_boss_ratio: getVal("vs_boss_ratio") / 100,
      vs_monster_ratio: getVal("vs_monster_ratio") / 100,
    },
    target: {
      def: getVal("target_def"),
      def_ratio: getVal("target_def_ratio") / 100,
      fortitude: getVal("target_fortitude"),
      crit_resist_rate: getVal("target_crit_resist_rate") / 100,
      crit_damage_resist_rate: getVal("target_crit_damage_resist_rate") / 100,
      hp_bonus_resist_rate: getVal("target_hp_bonus_resist_rate") / 100,
      is_boss: getCheck("target_is_boss"),
      is_high_hp: getCheck("target_is_high_hp"),
      has_bad_condition: getCheck("target_has_bad_condition"),
    }
  };
}

function displayResult(result: any) {
  const container = document.getElementById("result")!;
  const pct = (val: number) => `${(val * 100).toFixed(1)}%`;
  const fixed3 = (val: number) => val.toFixed(3);

  container.innerHTML = `
    <ul class="list-disc pl-5 space-y-1">
      <li><strong>非会心時:</strong> ${Math.floor(result.breakdown.non_crit_damage)}</li>
        <li><strong>会心時:</strong> ${result.breakdown.crit_rate === 0 ? "-" : Math.floor(result.breakdown.crit_damage)}</li>
        <li><strong>期待値:</strong> ${Math.floor(result.breakdown.expected_damage)}</li>
      </ul>
    <div class="text-sm mt-2 p-2 rounded">
      会心発生率: ${pct(result.breakdown.crit_rate)}<br>
      会心ダメージ倍率: ${pct(result.breakdown.crit_damage_ratio)}<br>
      高低異常枠合計: ${pct(result.breakdown.total_bonus_rate)}<br>
      有効防御率: ${pct(result.breakdown.effective_def_ratio)}<br>
      最終防御力: ${result.breakdown.modified_def}
    </div>
    <div class="text-sm mt-2 p-2 rounded">
      <h4 class="font-bold">1%あたりのダメージ上昇率:</h4>
      <div class="grid grid-cols-2 gap-1">
        <div>攻撃倍率: ${fixed3(result.sensitivity.atk_ratio_per_percent)}%</div>
        <div>威圧倍率: ${fixed3(result.sensitivity.intimidation_ratio_per_percent)}%</div>
        <div>ダメージ倍率: ${fixed3(result.sensitivity.damage_ratio_per_percent)}%</div>
        <div>会心率: ${fixed3(result.sensitivity.crit_rate_per_percent)}%</div>
        <div>会心ダメージ: ${fixed3(result.sensitivity.crit_damage_ratio_per_percent)}%</div>
        <div>高低異常枠合計: ${fixed3(result.sensitivity.total_bonus_rate_per_percent)}%</div>
        <div>対ボス/魔物与ダメ増: ${fixed3(result.sensitivity.vs_boss_or_monster_ratio_per_percent)}%</div>
      </div>
    </div>
  `;
}

run();
