import {test,expect} from '@playwright/test';
import {readFileSync} from 'node:fs';
async function ready(page){await page.goto('/SLATE/');await expect(page.getByRole('status')).toContainText('Comparison ready');}

test('actual WASM matches native canonical bytes for reset, outage and changed ratings',async({page})=>{
 await ready(page);const refs=JSON.parse(readFileSync('.cache/native-results.json','utf8'));
 const actual=await page.evaluate(async refs=>new Promise((resolve,reject)=>{const worker=new Worker('./worker.js',{type:'module'});const output=[];const timer=setTimeout(()=>{worker.terminate();reject(Error('parity timeout'));},10000);worker.onerror=()=>{clearTimeout(timer);worker.terminate();reject(Error('parity worker error'));};worker.onmessage=({data})=>{if(data.type==='ready')worker.postMessage({type:'evaluate',id:0,input:refs[0].input});else if(data.ok){output.push(data.canonical);if(output.length===refs.length){clearTimeout(timer);worker.terminate();resolve(output);}else worker.postMessage({type:'evaluate',id:output.length,input:refs[output.length].input});}else{clearTimeout(timer);worker.terminate();reject(Error(data.error));}};worker.postMessage({type:'init'});}),refs);
 expect(actual).toEqual(refs.map(ref=>ref.canonical));await expect(page.locator('#metric-rows tr')).toHaveCount(3);await expect(page.locator('#unknowns li')).toHaveCount(7);await expect(page.locator('#diagram circle')).toHaveCount(4);await expect(page.locator('#diagram line')).toHaveCount(3);
});

test('link controls, selected node, outage, download and reset retain unknown rights',async({page})=>{
 await ready(page);await page.getByRole('spinbutton',{name:'Rating (seats, Surge)'}).first().fill('240');await page.getByRole('spinbutton',{name:'Rating (seats, Surge)'}).first().press('Tab');await expect(page.getByRole('status')).toContainText('Comparison ready');await expect(page.locator('#metric-rows tr').nth(1).locator('td').last()).toContainText('240 seats');
 await page.getByRole('button',{name:'Try handoff loss'}).click();await expect(page.getByRole('status')).toContainText('Comparison ready');await expect(page.locator('#metric-rows tr').last().locator('td').last()).toContainText('Disconnected');
 const [download]=await Promise.all([page.waitForEvent('download'),page.getByRole('button',{name:'Download results JSON'}).click()]);const result=JSON.parse(readFileSync(await download.path(),'utf8'));expect(result.current.not_assessed.staffed_instruction).toBeNull();expect(result.current.authority.allocation).toBe(false);expect(result.current.metrics.path_connected_to_destination).toBe(false);
 await page.locator('#selected').selectOption('destination');await expect(page.getByRole('status')).toContainText('Comparison ready');await expect(page.locator('#node-context')).toContainText('destination');await expect(page.locator('#metric-rows tr').last()).toContainText('Connected');
 await page.getByRole('button',{name:'Reset graph'}).click();await expect(page.getByRole('status')).toContainText('Comparison ready');await expect(page.locator('#metric-rows tr').last().locator('td').last()).toContainText('Connected');
});

test('unknown, mixed, missing and oversized inputs clear stale results and recover',async({page})=>{
 await ready(page);await page.locator('#editor summary').click();const original=await page.locator('#input').inputValue();
 for(const input of [original.replace('"Surge"','"Baseline"'),original.replace('"capacity_seats": 120','"capacity_seats": null'),original.replace('"selected_node": "community"','"selected_node": "missing"'),original.replace('"destination": "destination"','"destination": "community"'),'{"person_name":"example"}','x'.repeat(65537)]){await page.locator('#input').fill(input);await expect(page.locator('#results')).toBeHidden();await expect(page.locator('#download')).toBeDisabled();await page.getByRole('button',{name:'Compare graph'}).click();await expect(page.getByRole('status')).toContainText(input.length>65536?'exceeds 64 KiB':'Cannot compare');}
 await page.getByRole('button',{name:'Reset graph'}).click();await expect(page.getByRole('status')).toContainText('Comparison ready');
});

test('loading and worker failure are readable',async({page})=>{
 await page.route('**/*.wasm',async route=>{await new Promise(resolve=>setTimeout(resolve,400));await route.abort();});await page.goto('/SLATE/');await expect(page.getByRole('status')).toContainText('Loading Rust');await expect(page.getByRole('status')).toContainText('Unable to load Rust WebAssembly');await expect(page.locator('#results')).toBeHidden();
});

test('keyboard, mobile and source/artifact routes work under SLATE prefix',async({page})=>{
 const errors=[];page.on('pageerror',error=>errors.push(error.message));await page.setViewportSize({width:390,height:844});await ready(page);await page.locator('#compare').focus();await page.keyboard.press('Tab');await expect(page.locator('#reset')).toBeFocused();await page.keyboard.press('Enter');await expect(page.getByRole('status')).toContainText('Comparison ready');expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
 for(const file of ['pkg/slate_web.wasm','example.json','outage.json','LICENSE','THIRD-PARTY-LICENSES.txt','provenance.json'])expect((await page.request.get('/SLATE/'+file)).ok()).toBe(true);await expect(page.locator('#taxlane')).toHaveAttribute('href','https://giodl73-repo.github.io/TAXLANE/#evidence-education');expect(errors).toEqual([]);
});

test('a newly added node has an explicitly unavailable reset observation',async({page})=>{
 await ready(page);await page.locator('#editor summary').click();const input=JSON.parse(await page.locator('#input').inputValue());input.nodes.push({id:'new',name:'Cedar-new',role:'Secondary'});input.selected_node='new';await page.locator('#input').fill(JSON.stringify(input));await page.getByRole('button',{name:'Compare graph'}).click();await expect(page.getByRole('status')).toContainText('Comparison ready');await expect(page.locator('#metric-rows')).toContainText('Not observed');
 const [download]=await Promise.all([page.waitForEvent('download'),page.getByRole('button',{name:'Download results JSON'}).click()]);const result=JSON.parse(readFileSync(await download.path(),'utf8'));expect(result.baseline).toBeNull();expect(result.current.not_assessed.learning_attainment_mobility).toBeNull();
});


test('more seat ratings leave the broken handoff disconnected',async({page})=>{await ready(page);await page.getByRole('button',{name:'Try handoff loss'}).click();await expect(page.getByRole('status')).toContainText('Comparison ready');const rating=page.getByRole('spinbutton',{name:'Rating (seats, Surge)'}).first();await rating.fill('240');await rating.press('Tab');await expect(page.getByRole('status')).toContainText('Comparison ready');await expect(page.locator('#metric-rows tr').nth(1).locator('td').last()).toContainText('240 seats');await expect(page.locator('#metric-rows tr').last().locator('td').last()).toContainText('Disconnected');});
