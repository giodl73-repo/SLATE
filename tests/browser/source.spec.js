import {test,expect} from '@playwright/test';
import {readFileSync} from 'node:fs';
async function ready(page){await page.goto('/SLATE/');await expect(page.locator('#source-status')).toContainText('Source ready');await expect(page.locator('#status')).toContainText('Comparison ready');}
async function download(page){const [file]=await Promise.all([page.waitForEvent('download'),page.locator('#source-download').click()]);return JSON.parse(readFileSync(await file.path(),'utf8'));}

test('actual WASM matches native canonical source queries for US California Tennessee and Guam',async({page})=>{
 await ready(page);const refs=JSON.parse(readFileSync('.cache/nces-native-results.json','utf8'));
 const actual=await page.evaluate(async refs=>new Promise((resolve,reject)=>{const worker=new Worker('./worker.js',{type:'module'}),output=[];const timer=setTimeout(()=>{worker.terminate();reject(Error('source parity timeout'));},10000);worker.onerror=()=>{clearTimeout(timer);worker.terminate();reject(Error('source worker failed'));};worker.onmessage=({data})=>{if(data.type==='ready')worker.postMessage({type:'evaluate',id:0,input:refs[0].input});else if(data.ok){output.push(data.canonical);if(output.length===refs.length){clearTimeout(timer);worker.terminate();resolve(output);}else worker.postMessage({type:'evaluate',id:output.length,input:refs[output.length].input});}else{clearTimeout(timer);worker.terminate();reject(Error(data.error));}};worker.postMessage({type:'init'});}),refs);
 expect(actual).toEqual(refs.map(ref=>ref.canonical));await expect(page.locator('#source-jurisdiction option')).toHaveCount(59);await expect(page.locator('#source-rows tr')).toHaveCount(5);
});

test('US enrollment change is observed but absent national staff and ratio remain null',async({page})=>{
 await ready(page);await expect(page.locator('#source-rows tr').nth(2)).toContainText('49,404,386');await expect(page.locator('#source-rows tr').nth(2)).toContainText('49,269,882');await expect(page.locator('#source-rows tr').nth(2)).toContainText('-134,504 students');await expect(page.locator('#source-rows tr').nth(3)).toContainText('Withheld');
 const result=await download(page);expect(result.observations[0].values.teacher_fte.value).toBe(3253091);expect(result.observations[1].values.teacher_fte.value).toBeNull();expect(result.changes_2024_25_minus_2023_24.teacher_fte.value).toBeNull();expect(result.changes_2024_25_minus_2023_24.pupil_teacher_ratio.value).toBeNull();expect(result.state_dc_coverage[1].fields.teacher_fte.reported_rows).toBe(50);expect(result.state_dc_coverage[1].fields.teacher_fte.complete_state_dc_sum).toBeNull();expect(result.state_dc_coverage[1].fields.teacher_fte.missing_jurisdictions).toEqual(['Tennessee']);expect(Object.values(result.not_assessed).every(v=>v===null)).toBe(true);expect(Object.values(result.authority).every(v=>v===false)).toBe(true);
});

test('matched California source values retain precision and do not alter the fictional graph',async({page})=>{
 await ready(page);const graph=await page.locator('#metric-rows').textContent();await page.locator('#source-jurisdiction').selectOption('California');await expect(page.locator('#source-status')).toContainText('Source ready');await expect(page.locator('#source-scope')).toContainText('State/DC aggregate');await expect(page.locator('#source-rows tr').nth(3)).toContainText('-251');await expect(page.locator('#source-rows tr').nth(4)).toContainText('-0.1 ratio points');
 const result=await download(page);expect(result.observations[0].values.student_membership.value).toBe(5837338);expect(result.observations[1].values.student_membership.value).toBe(5805510);expect(result.observations[1].values.pupil_teacher_ratio.published).toBe('21.7');expect(result.not_assessed.class_size).toBeNull();expect(await page.locator('#metric-rows').textContent()).toBe(graph);
});

test('Tennessee missing staff and Guam suppression preserve the distinct source markers',async({page})=>{
 await ready(page);await page.locator('#source-tennessee').click();await expect(page.locator('#source-status')).toContainText('Source ready');let result=await download(page);expect(result.observations[1].values.teacher_fte.published).toBe('—');expect(result.changes_2024_25_minus_2023_24.teacher_fte.value).toBeNull();
 await page.locator('#source-guam').click();await expect(page.locator('#source-status')).toContainText('Source ready');result=await download(page);expect(result.population_group).toBe('other_jurisdiction');expect(result.observations[0].values.student_membership.published).toBe('—');expect(result.observations[1].values.student_membership.published).toBe('‡');expect(result.changes_2024_25_minus_2023_24.student_membership.value).toBeNull();expect(result.sources[1].notes.join(' ')).toContain('suppressed due to data quality concerns');await expect(page.locator('#source-scope')).toContainText('excluded from US totals');
 await page.locator('#source-reset').click();await expect(page.locator('#source-status')).toContainText('Source ready');await expect(page.locator('#source-jurisdiction')).toHaveValue('United States');
});

test('unsupported source inputs invalidate stale source output and leave the graph independent',async({page})=>{
 await ready(page);await page.locator('#source-editor summary').click();for(const input of ['{"view":"nces-state-comparison","jurisdiction":"XX"}','{"view":"nces-state-comparison","jurisdiction":"California","student_name":"Example"}','{"view":"nces-state-comparison","jurisdiction":"California","teacher_fte":0}','x'.repeat(65537)]){await page.locator('#source-query').fill(input);await expect(page.locator('#source-results')).toBeHidden();await expect(page.locator('#source-download')).toBeDisabled();await page.locator('#source-run').click();await expect(page.locator('#source-status')).toContainText(input.length>65536?'exceeds 64 KiB':'Cannot query source');await expect(page.locator('#results')).toBeVisible();}
 await page.locator('#source-reset').click();await expect(page.locator('#source-status')).toContainText('Source ready');
});

test('keyboard mobile source hashes and artifact routes work under the project prefix',async({page})=>{
 const errors=[];page.on('pageerror',e=>errors.push(e.message));await page.setViewportSize({width:390,height:844});await ready(page);await page.locator('#source-run').focus();await page.keyboard.press('Tab');await expect(page.locator('#source-reset')).toBeFocused();await page.keyboard.press('Enter');await expect(page.locator('#source-status')).toContainText('Source ready');expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
 const custody=await(await page.request.get('/SLATE/source-custody.json')).json();expect(custody.sources).toHaveLength(2);for(const source of custody.sources){const actual=await page.evaluate(async path=>{const bytes=await(await fetch(path)).arrayBuffer();const hash=await crypto.subtle.digest('SHA-256',bytes);return Array.from(new Uint8Array(hash),byte=>byte.toString(16).padStart(2,'0')).join('');},source.published);expect(actual).toBe(source.sha256);}
 for(const file of ['nces-summary.json','SOURCE-NOTICE.txt','pkg/slate_web.wasm','provenance.json'])expect((await page.request.get('/SLATE/'+file)).ok()).toBe(true);await expect(page.locator('#taxlane')).toHaveAttribute('href','https://giodl73-repo.github.io/TAXLANE/#evidence-education-training-employment-social-services');expect(errors).toEqual([]);
});

test('source-engine loading failure disables source export and remains readable',async({page})=>{
 await page.route('**/*.wasm',async route=>{await new Promise(resolve=>setTimeout(resolve,400));await route.abort();});await page.goto('/SLATE/');await expect(page.locator('#source-status')).toContainText('Loading source engine');await expect(page.locator('#source-status')).toContainText('Unable to load the source engine');await expect(page.locator('#source-results')).toBeHidden();await expect(page.locator('#source-download')).toBeDisabled();
});
