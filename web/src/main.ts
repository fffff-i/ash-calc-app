import init, { calc_damage_verbose_js } from './pkg/ash_calc_app';
import "./style.css";

async function run() {
  await init();
  console.log("WASM module initialized.");

  document.getElementById("calc-button")!.addEventListener("click", () => {
    const params = getParamsFromUI();
    const breakdown = calc_damage_verbose_js(params, []);
    displayResult(breakdown);
  });
}

function getParamsFromUI() {
  const getVal = (id: string) => parseFloat((document.getElementById(id) as HTMLInputElement).value);
  const getCheck = (id: string) => (document.getElementById(id) as HTMLInputElement).checked;

  const rawAtk = getVal("atk") / (getVal("atk_bonus") / 100.0);
  const rawIntimidation = getVal("intimidation") / (getVal("intimidation_rate") / 100.0 + 1.0);

  return {
    attacker: {
      atk: rawAtk,
      def_penetration_rate: getVal("def_penetration_rate") / 100,
      intimidation: rawIntimidation,
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

function displayResult(breakdown: any) {
  const container = document.getElementById("result")!;
  const pct = (val: number) => `${(val * 100).toFixed(1)}%`;
  const fixed1 = (val: number) => val.toFixed(1);

  container.innerHTML = `
      <ul class="list-disc pl-5 space-y-1">
        <li><strong>非会心時:</strong> ${Math.floor(breakdown.non_crit_damage)}</li>
        <li><strong>会心時:</strong> ${breakdown.crit_rate === 0 ? "-" : Math.floor(breakdown.crit_damage)}</li>
        <li><strong>期待値:</strong> ${Math.floor(breakdown.expected_damage)}</li>
      </ul>
    <div class="text-sm mt-2">
      会心発生率: ${pct(breakdown.crit_rate)}<br>
      会心ダメージ倍率: ${pct(breakdown.crit_damage_ratio)}<br>
      高低異常枠合計: ${pct(breakdown.total_bonus_rate)}<br>
      有効防御率: ${pct(breakdown.effective_def_ratio)}<br>
      最終防御力: ${breakdown.modified_def}
    </div>
    <div class="text-sm mt-4 p-2 bg-gray-100 rounded">
      <h4 class="font-bold">1%あたりのダメージ上昇率:</h4>
      <div class="grid grid-cols-2 gap-1">
        <div>攻撃倍率: ${fixed1(breakdown.sensitivity.atk_ratio_per_percent)}%</div>
        <div>威圧倍率: ${fixed1(breakdown.sensitivity.intimidation_ratio_per_percent)}%</div>
        <div>会心率: ${fixed1(breakdown.sensitivity.crit_rate_per_percent)}%</div>
        <div>会心ダメージ: ${fixed1(breakdown.sensitivity.crit_damage_ratio_per_percent)}%</div>
        <div>高低異常枠合計: ${fixed1(breakdown.sensitivity.total_bonus_rate_per_percent)}%</div>
        <div>対ボス/魔物与ダメ増: ${fixed1(breakdown.sensitivity.vs_boss_or_monster_ratio_per_percent)}%</div>
      </div>
    </div>
  `;
}

run();
