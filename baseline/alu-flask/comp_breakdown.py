import json

with open("/home/yjn/rust_project/mslibos/baseline/alu-flask/alu-flask-remote-0kb.trace", 'r') as f:
    trace_info = json.loads(f.readline())
    func_info = json.loads(f.readline())

batch_size = len(trace_info['invoke'])

cli_to_gateway = [(trace_info['gateway-receive-request'][i]-trace_info['invoke'][i]) + (trace_info['finish'][i]-trace_info['gateway-proxy-resp'][i])
                  for i in range(batch_size)]
cli_to_gateway = sum(cli_to_gateway) / batch_size
print("cli_to_gateway:", cli_to_gateway)

gateway_to_netes = [(trace_info['netes-resolve-addr'][i]-trace_info['gateway-receive-request'][i]) + (trace_info['gateway-proxy-resp'][i]-trace_info['netes-func-response'][i])
                    for i in range(batch_size)]
gateway_to_netes = sum(gateway_to_netes) / batch_size
print("gateway_to_netes:", gateway_to_netes)

netes_to_watchdog = [(trace_info['watchdog_begin'][i]-trace_info['netes-resolve-addr'][i]) + (trace_info['netes-func-response'][i]-trace_info['watchdog-resp'][i])
                     for i in range(batch_size)]
netes_to_watchdog = sum(netes_to_watchdog) / batch_size
print("netes_to_watchdog:", netes_to_watchdog)

# watchdog_to_func = [(trace_info['watchdog-resp'][i]-trace_info['watchdog_begin'][i]) - trace_info['X-Duration-Useconds'][i]
#                     for i in range(batch_size)]
watchdog_to_func = [(trace_info['watchdog-resp'][i]-trace_info['watchdog_begin'][i]) - func_info['comp_time'][i]
                    for i in range(batch_size)]
watchdog_to_func = sum(watchdog_to_func) / batch_size
print("watchdog_to_func:", watchdog_to_func)

func_comp = sum(func_info['comp_time'])/batch_size
print("func_comp:", func_comp)